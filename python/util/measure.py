import time


def timeit(method):
    def timed(*args, **kw):
        ts = time.perf_counter_ns()
        result = method(*args, **kw)
        te = time.perf_counter_ns()
        elapsed_time = (te - ts) / 1e9  # ns to s
        print(f"{method.__name__} took: {elapsed_time} seconds")
        return result

    return timed
