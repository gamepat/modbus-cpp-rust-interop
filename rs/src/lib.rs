use anyhow::Result;
use tokio_modbus::client::sync::{Reader, Writer, Context};

#[cxx::bridge]
mod ffi {
    extern "Rust" {

        fn hello() -> ();

        #[namespace = "modbus"]
        type ModbusClient;

        #[namespace = "modbus"]
        fn create(addr: String) -> Result<Box<ModbusClient>>;

        #[namespace = "modbus"]
        fn read_input_registers(&mut self, addr: u16, cnt: u16) -> Result<Vec<u16>>;

        #[namespace = "modbus"]
        fn read_holding_registers(&mut self, addr: u16, cnt: u16) -> Result<Vec<u16>>;

        #[namespace = "modbus"]
        fn write_registers(&mut self, addr: u16, data: &[u16]) -> Result<()>;
    }
}

fn hello() -> () {
    println!("Hello world from rust");
}

struct ModbusClient {
    ctx: Context,
}

fn create(addr: String) -> Result<Box<ModbusClient>> {
    use tokio_modbus::prelude::*;

    let socket_addr = addr.parse()?;

    println!("Creating context ...");
    let ctx = sync::tcp::connect_with_timeout(socket_addr, Some(std::time::Duration::from_secs(1)))?;
    println!("Created context");

    Ok(Box::new(ModbusClient{ctx}))
}

impl ModbusClient {
    fn read_input_registers(&mut self, addr: u16, cnt: u16) -> Result<Vec<u16>> {
        println!("Reading input registers ...");
        let buff = self.ctx.read_input_registers(addr, cnt)?;
        println!("Got registers: {:?}", buff);
        Ok(buff)
    }
    
    fn read_holding_registers(&mut self, addr: u16, cnt: u16) -> Result<Vec<u16>> {
        println!("Reading holding registers ...");
        let buff = self.ctx.read_holding_registers(addr, cnt)?;
        println!("Got registers: {:?}", buff);
        Ok(buff)
    }
    
    fn write_registers(&mut self, addr: u16, data: &[u16]) -> Result<()> {
        println!("Writing registers ...");
        self.ctx.write_multiple_registers(addr, data)?;
        println!("Write registers finished!");
        Ok(())
    }
}
