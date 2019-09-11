'use strict';

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.ID = ID;
exports.Bar = Bar;
exports.Foo = Foo;
exports.FooStatus = FooStatus;

var _xdrJsSerialize = require('xdr-js-serialize');

var _xdrJsSerialize2 = _interopRequireDefault(_xdrJsSerialize);

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

// Namespace start example

// Start typedef section

function ID() {
  return new _xdrJsSerialize2.default.FixedOpaque(32);
}
// End typedef section

// Start struct section
function Bar() {
  return new _xdrJsSerialize2.default.Struct(['id'], [ID()]);
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
function Foo() {
  return new _xdrJsSerialize2.default.Struct(['status', 'one', 'two', 'three'], [FooStatus(), new _xdrJsSerialize2.default.Str('', 256), new _xdrJsSerialize2.default.Str('', 256), new _xdrJsSerialize2.default.Str('', 256)]);
}

// End struct section

// Start enum section

function FooStatus() {
  return new _xdrJsSerialize2.default.Enum({
    0: 'Zero',
    1: 'One',
    2: 'Two',
    3: 'Three'

  });
}

// End enum section

// Start union section

// End union section

// End namespace example