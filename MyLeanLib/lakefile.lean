import Lake
open Lake DSL

package «MyLeanLib» where
  -- add package configuration options here

lean_lib «MyLeanLib» where
  -- add library configuration options here

@[default_target]
lean_exe «myleanlib» where
  root := `Main
