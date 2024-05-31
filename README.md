# Lang Basics &emsp; [![sh-python]][python] [![sh-rust]][rust] [![sh-go]][go]

[sh-python]:https://img.shields.io/badge/Python-3.7-white?style=flat-square&logo=Python&logoColor=white&labelColor=blue
[python]:https://python.org/
[sh-rust]:https://img.shields.io/badge/Rust-1.78.0-white?style=flat-square&logo=Rust&logoColor=white&labelColor=orange
[rust]:https://example.com
[sh-go]:https://img.shields.io/badge/Go-1.22.3-white?style=flat-square&logo=Go&logoColor=white&labelColor=blue
[go]:https://example.com

Collection of examples for common programming language tasks through the
implementation of a simple Web server library.

## Objectives

By implementing a Web server library it is expected to learn about these items:

- Use the language development tools (package manager, lang-server, linter, formatter, docs generator, ...)
- Build and run the source code
- Create and import external libraries
- Unit testing
- Read and write operations on sockets and files.
- String manipulation
- Error handling

## Specs

In order to meet the requirements mentioned above, each implementation tries to follow this specification:

### 1. Project structure

   The implementation includes a external library and a main application. So this structure somehow emulate
   handle both cases cases.

   ```
    .
    ├── app
    │   ├── src    -> main app source
    │   └── tests
    ├── web
    │   ├── docs   -> used to generate docs if available
    │   ├── src    -> library source
    │   └── tests
    └── README.md  -> Explain how to compile/run, install, test
                      and do other task for the app/library
   ```

### 2. Library API

  The library will create an Web server leaving the handling of resources to the app. The app should be
  able to import and use the library in a way similar to this:

  ```
  IMPORT web

  app = web.App(CUSTOM_HOST, CUSTOM_PORT)

  HOME_VIEW(request, [VARIADIC_ARGS]):
    IF request.method == "GET":
      RETURN DO_SOMETHING_WITH(request.data)
    ELSE:
      RETURN DEFAULT_RESPONSE()

  ANOTHER_VIEW(request, [VARIADIC_ARGS]):
    RETURN DO_SOMETHING_WITH(request.data) + VARIADIC_ARGS.id

  NOTFOUND_VIEW(error_code, request):
    RETURN DO_SOMETHING_WITH(error_code)

  MAIN():
    app.register("", HOME_VIEW)
    app.register("goto/<id>/", ANOTHER_VIEW)
    app.errorhandler(NOTFOUND_VIEW)
    app.run()
  ```

  The actual implementation will differ but the concept should remain the same. 
  So in the end the library should exposed as minimum four things:
  
  1. A constructor for web server with an optional HOST and PORT number
  2. A register function of routes to the web server. Should take a route pattern
     and a handler that takes a request context and variadic arguments depending on the route pattern.
     The handler should respond with a response context.
  3. A Register function for handlers that respond with errors.
  4. A function to start listening to requests.

> [!TIP]
> Essentially what the library does is receive requests, select the correct handler, pass it the arguments
> on the route pattern, and respond with the results of said handler.

### 3. Main application

  The application will add the library as a dependency using any prefered strategy for the programming 
  language such as packagage managers or scripts and create a simple web server using the library.

  The application should create a command line interface to start the server with a custom port.

  ```bash
  ./simplewebserver -P 8000
  ```
### 4. Misc

  The implementation will also include these things:

  - A generated documentation for the library
  - A suite of test cases for both the library and the application

## License

[MIT license](./LICENSE)
