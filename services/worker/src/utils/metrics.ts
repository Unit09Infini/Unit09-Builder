export interface Counter {
  inc(value?: number): void;
  get(): number;
}

export interface Gauge {
  set(value: number): void;
  get(): number;
}

export interface WorkerMetrics {
  jobsStarted: Counter;
  jobsCompleted: Counter;
  jobsFailed: Counter;
  activeJobs: Gauge;
}

function createCounter(): Counter {
  let value = 0;
  return {
    inc(delta = 1) {
      value += delta;
    },
    get() {
      return value;
    }
  };
}

function createGauge(): Gauge {
  let value = 0;
  return {
    set(v: number) {
      value = v;
    },
    get() {
      return value;
    }
  };
}

export function createWorkerMetrics(): WorkerMetrics {
  return {
    jobsStarted: createCounter(),
    jobsCompleted: createCounter(),
    jobsFailed: createCounter(),
    activeJobs: createGauge()
  };
}
