use prelude::*;
use futures_cpupool::CpuPool;

pub trait ZirconApp : Send + Sync + 'static {
    /// Returns the number of accept threads.
    fn num_accept_threads(&self) -> usize;
    /// Returns the number of cpu threads.
    fn num_cpu_threads(&self) -> usize;
    /// Returns the internal cpu pool (not for accept threads).
    fn cpu_pool(&self) -> &CpuPool;
}

#[derive(Clone)]
pub struct ZirconDefaultApp<D: Send + Sync + 'static> {
    pub config: ZirconConfig,
    pub cpu_pool: CpuPool,
    pub server_data: D,
}

impl<D: Send + Sync + 'static> ZirconDefaultApp<D> {
    pub fn from_config(config: ZirconConfig) -> ZirconDefaultApp<()> {
        let cpu_threads = config.num_cpu_threads();
        ZirconDefaultApp {
            config: config,
            cpu_pool: CpuPool::new(cpu_threads),
            server_data: (),
        }
    }

    pub fn with_server_data<D2: Send + Sync + 'static>(self, data: D2) -> ZirconDefaultApp<D2> {
        ZirconDefaultApp {
            config: self.config,
            cpu_pool: self.cpu_pool,
            server_data: data,
        }
    }
}

impl<D: Send + Sync + 'static> ZirconApp for ZirconDefaultApp<D> {
    fn num_accept_threads(&self) -> usize {
        self.config.num_accept_threads()
    }

    fn num_cpu_threads(&self) -> usize {
        self.config.num_cpu_threads()
    }

    fn cpu_pool(&self) -> &CpuPool {
        &self.cpu_pool
    }
}
