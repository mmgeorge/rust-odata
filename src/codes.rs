

mode codes {
    // Success Codes
    const OK: u16 = 200;
    const CREATED: u16 = 201;
    const ACCEPTED: u16 = 202;
    const NO_CONTENT: u16 = 204;
    const NOT_MODIFIED: u16 = 304;

    // Error Codes
    const BAD_REQUEST: u16 = 400;
    const FORBIDDEN: u16 = 403; 
    const NOT_FOUND: u16 = 404;
    const METHOD_NOT_ALLOWED: u16 = 405;
    const GONE: u16 = 410;
    const PRECONDITION_FAILEd: u16 = 412;
    const NOT_IMPL: u16 = 501;
}
