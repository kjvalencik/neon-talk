use neon::prelude::*;

mod middleware;
mod sieve;

#[derive(Debug)]
struct CountPrimesTask {
	max: usize,
}

impl Task for CountPrimesTask {
	type Output = usize;
	type Error = ();
	type JsEvent = JsNumber;

	fn perform(&self) -> Result<Self::Output, Self::Error> {
		let count = sieve::sieve(self.max).count();

		Ok(count)
	}

	fn complete(
		self,
		mut cx: TaskContext,
		result: Result<Self::Output, Self::Error>,
	) -> JsResult<Self::JsEvent> {
		let count = match result {
			Ok(count) => count,
			Err(_) => {
				return cx.throw_error("Unreachable error in CountPrimesTask")
			}
		};

		Ok(cx.number(count as f64))
	}
}

fn count_primes(mut cx: FunctionContext) -> JsResult<JsNumber> {
	let max = cx.argument::<JsNumber>(0)?.value() as usize;

	let count = sieve::sieve(max).count();

	Ok(cx.number(count as f64))
}

fn count_primes_async(mut cx: FunctionContext) -> JsResult<JsUndefined> {
	let max = cx.argument::<JsNumber>(0)?.value() as usize;
	let cb = cx.argument::<JsFunction>(1)?;

	(CountPrimesTask { max }).schedule(cb);

	Ok(cx.undefined())
}

register_module!(mut cx, {
	cx.export_function("countPrimesSync", count_primes)?;
	cx.export_function("countPrimesAsync", count_primes_async)?;
	cx.export_function(
		"countPrimesMiddleware",
		middleware::count_primes_middleware,
	)?;

	Ok(())
});
