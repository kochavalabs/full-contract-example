
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});

var _xdrJsSerialize = _interopRequireDefault(require("xdr-js-serialize"));
function _interopRequireDefault(obj) {
    return obj && obj.__esModule ? obj : {
        default: obj
    };
}

// Namespace start example

// Start typedef section
exports.ID = ID;
function ID() {
    return new _xdrJsSerialize.default.FixedOpaque(32);
}
// End typedef section

// Start struct section
exports.Bar = Bar;
function Bar() {
    return new _xdrJsSerialize.default.Struct(
        ["id",],
        [ID(),]
    )
}
// End struct section

// Start enum section

// End enum section

// Start union section

// End union section

// End namespace example
// Namespace start example

// Start typedef section
// End typedef section

// Start struct section
exports.Foo = Foo;
function Foo() {
    return new _xdrJsSerialize.default.Struct(
        ["status","one","two","three",],
        [FooStatus(),new _xdrJsSerialize.default.Str('', 256),new _xdrJsSerialize.default.Str('', 256),new _xdrJsSerialize.default.Str('', 256),]
    )
}
// End struct section

// Start enum section
exports.FooStatus = FooStatus;
function FooStatus() {
    return new _xdrJsSerialize.default.Enum({
0: "Zero",
1: "One",
2: "Two",
3: "Three",
    })
}

// End enum section

// Start union section

// End union section

// End namespace example
