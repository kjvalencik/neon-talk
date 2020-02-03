use neon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::sieve::sieve;

#[derive(Debug, Deserialize)]
struct CountPrimesRequest {
	max: usize,
}

#[derive(Debug, Serialize)]
struct CountPrimesResponse {
	count: usize,
}

struct CountPrimesMiddlewareTask {
	body: Vec<u8>,
}

impl Task for CountPrimesMiddlewareTask {
	type Output = Vec<u8>;
	type Error = serde_json::Error;
	type JsEvent = JsBuffer;

	fn perform(&self) -> Result<Self::Output, Self::Error> {
		let req: CountPrimesRequest =
			serde_json::from_slice(self.body.as_slice())?;

		let count = sieve(req.max).count();
		let res = CountPrimesResponse { count };

		Ok(serde_json::to_vec(&res)?)
	}

	fn complete(
		self,
		mut cx: TaskContext,
		result: Result<Self::Output, Self::Error>,
	) -> JsResult<Self::JsEvent> {
		let res = match result {
			Ok(count) => count,
			Err(err) => return cx.throw_error(err.to_string()),
		};

		let buf = cx.buffer(res.len() as u32)?;

		cx.borrow(&buf, |contents| {
			contents.as_mut_slice().copy_from_slice(&res)
		});

		Ok(buf)
	}
}

pub fn count_primes_middleware(
	mut cx: FunctionContext,
) -> JsResult<JsUndefined> {
	let req = cx.argument::<JsBuffer>(0)?;
	let cb = cx.argument::<JsFunction>(1)?;
	let body = cx.borrow(&req, |contents| contents.as_slice().to_vec());

	(CountPrimesMiddlewareTask { body }).schedule(cb);

	Ok(cx.undefined())
}
