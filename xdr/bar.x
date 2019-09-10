// We currently ignore namespace, but they are required by the grammar.
namespace example
{

  enum FooStatus {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
  };


  struct Foo {
    FooStatus status;

    string one<256>;

    string two<256>;

    string three<256>;
  };

}
