#include <stdio.h>
#include <vector>
#include <unordered_map>
#include <string>
#include <stdint.h>

using namespace std;

typedef enum {
    None,
    True,
    False,
} TriBool;

struct PulseCounter {
    PulseCounter() : low(0), high(0) {}

    void pulse(bool value) {
        if (value) high++; else low++;
    }

    void reset() {
        low = 0; high = 0;
    }

    int64_t low, high;
};

PulseCounter pc;

struct Module {
    Module(const char *n): name(n) {}

    virtual TriBool perform(bool value, Module* source) { return value ? True : False; }

    virtual void pulse_in(bool value, Module* source) {
        pc.pulse(value);
        TriBool result = perform(value, source);
        if (result == None) return;
        for (auto output : outputs) {
            output->pulse_in(result == True, this);
        }
    }

    virtual void debug() {
        printf("%s\n", name);
    }

    virtual void add_input(Module* input) {
        inputs.push_back(input);
    }

    virtual void add_output(Module* output) {
        outputs.push_back(output);
    }

    virtual int input_index(Module* input) {
        for (int i = 0; i < inputs.size(); i++) {
            if (inputs[i] == input) return i;
        }
        return -1;
    }

    const char *name;
    std::vector<Module*> inputs;
    std::vector<Module*> outputs;
    std::vector<int> output_indices;
};

struct BroadcastModule : public Module {
    BroadcastModule(): Module("Broadcast") {}

    virtual void pulse_in(bool value, Module* source) {
        pc.pulse(value);
        for (auto output : outputs) {
            output->pulse_in(value, this);
        }
    }
};

struct FlipFlopModule : public Module {
    FlipFlopModule(): Module("FlipFlop"), cur_value(false) {}

    bool cur_value;

    virtual TriBool perform(bool value, Module* source) {
        if (value) return None;

        cur_value = !cur_value;
        return cur_value ? True : False;
    }
};

struct ConjunctionModule : public Module {
    ConjunctionModule(): Module("Conjunction") {}

    std::vector<bool> input_values;

    virtual TriBool perform(bool value, Module* source) override {
        int index = input_index(source);
        if (index == -1) return None;

        input_values[index] = value;

        for (auto ival : input_values) {
            if (!ival) return True;
        }
        return False;
    }

    virtual void debug() override {
        printf("%s: ", name);
        for (int i = 0; i < inputs.size(); i++) {
            printf("%p - %s ", inputs[i], input_values[i] ? "true" : "false");
        }
        printf("\n");
    }

    virtual void add_input(Module* input) override {
        Module::add_input(input);
        input_values.push_back(false);
    }
};

int main(int argc, char** argv) {
    std::unordered_map<std::string, int> module_name_to_index;
    std::vector<Module*> modules;

    auto index_for_name = [&](const char *name) {
        if (module_name_to_index.find(name) != module_name_to_index.end()) {
            return module_name_to_index[name];
        } else {
            int index = modules.size();
            module_name_to_index[name] = index;
            modules.push_back(nullptr);
            return index;
        }
    };

    Module* broadcast = nullptr;

    // open inputs/day20-sample.txt and read lines
    FILE *fp = fopen("inputs/day20-sample.txt", "r");
    char line[256];
    while (fgets(line, sizeof(line), fp)) {
        printf("%s", line);

        // lines look like:
        // &name -> a, b, c
        //
        // parse out the name and the a b c names
        char front[256];
        char outputs[256];
        sscanf(line, "%s -> %s", front, outputs);

        char *name = front+1;
        Module *m = nullptr;

        if (front[0] == '%') {
            m = new FlipFlopModule();
        } else if (front[0] == '&') {
            m = new ConjunctionModule();
        } else if (front[0] == 'b') {
            m = new BroadcastModule();
            broadcast = m;
            name = front;
        } else {
            printf("unknown module: %s\n", front);
            exit(1);
        }

        int module_index = index_for_name(name);
        modules[module_index] = m;

        // split out inputs by comma
        char *output = strtok(outputs, ",");
        while (output != NULL) {
            printf("output: %s\n", output);
            int output_index = index_for_name(output);
            m->output_indices.push_back(output_index);

            output = strtok(NULL, ",");
        }
    }

printf("====\n");
    for (auto module : modules) {
        for (auto mi : module->output_indices) {
            module->add_output(modules[mi]);
            module->debug();
        }
    }

    broadcast->pulse_in(false, nullptr);

    printf("low: %lld, high: %lld mult: %lld\n", pc.low, pc.high, pc.low * pc.high);
}
