# Copyright © 2020 Lexi Frydl
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# Displays information about available recipes.
help:
  just --list

# Compiles shaders to SPIR-V.
compile-shaders:
  #!/bin/fish

  set -l cwd (pwd)

  for shader in **/*.vert **/*.frag
    cd $cwd/(dirname $shader)
    and glslc -c *.vert *.frag
    or exit 1
  end

# Adds the MPL license header to all source files.
license:
  #!/bin/fish

  set -l notice \
  "// Copyright © 2020 Lexi Frydl
  //
  // This Source Code Form is subject to the terms of the Mozilla Public
  // License, v. 2.0. If a copy of the MPL was not distributed with this
  // file, You can obtain one at http://mozilla.org/MPL/2.0/.
  "

  for file in **/*.rs
    if string match -q 'structopt/*' $file
      continue
    else if grep -q "http://mozilla.org/MPL/2.0/" $file
      continue
    end

    echo $notice > $file.tmp
    and cat $file >> $file.tmp
    and mv $file{.tmp,}
  end

publish:
  cd proc-macros && cargo publish --target-dir ../target
  cd macros && cargo publish --target-dir ../target
  cd structopt/structopt-derive && cargo publish --target-dir ../../target
  cd structopt && cargo publish --target-dir ../target
  cargo publish
