# Austenite

A library for building [Iron][iron] handlers that implements HTTP header
handling and content negotiation.

A port of Clojure's [Liberator][liberator] library, itself inspired by
Erlang's [webmachine][webmachine].

## Install

Add `austenite = "*"` to your `Cargo.toml` dependencies.

## Usage

```rust
struct GetOkContent;
resource_handler!(GetOkContent);

impl Resource for GetOkContent {
    fn handle_ok(&self, req: &Request, resp: &mut Response)
               -> IronResult<Response>
    {
      resp.set_mut((status::Ok, "hello"));
      Ok(Response::new())
    }
}

…
Iron::new(Resource).listen((address,0u16));
…
```

## License

TBD

[iron]:https://github.com/iron/iron "Iron HTTP middleware and modifier library"
[liberator]:http://clojure-liberator.github.io/liberator/ "Liberator"
[webmachine]:https://github.com/basho/webmachine "webmachine"
