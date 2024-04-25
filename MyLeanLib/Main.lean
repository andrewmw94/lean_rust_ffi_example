import «MyLeanLib»

def foo : String :=
  myOtherFunNoArg

def main : IO Unit :=
  IO.println s!"{foo}"
