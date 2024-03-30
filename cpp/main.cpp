#include <iostream>
#include <vector>

#include "rs/src/lib.rs.h"

int main() {
    std::cout << "Hello, World!" << std::endl;
    hello();

    auto client = modbus::create("127.0.0.1:5502");

    auto input_reg = client->read_input_registers(0, 2);

    auto holding_reg = client->read_holding_registers(0, 4);

    std::vector<uint16_t> x{2, 3};
    rust::Slice<const uint16_t> s(x.data(), x.size());
    client->write_registers(1, s);

    auto holding_reg2 = client->read_holding_registers(0, 4);

    return 0;
}
