{ pkgs }:
let
  testFiles = files: relPath:
    let
      filesHead = builtins.head files;
      isDirectory = (builtins.substring ((builtins.stringLength filesHead) - 1) 1 filesHead) == "/";
      current =
        if isDirectory
        then builtins.substring 0 ((builtins.stringLength filesHead) - 1) filesHead
        else filesHead;
    in
    files != [ ] && (
      relPath == current ||
      (isDirectory && builtins.match "${current}/.*" relPath != null) ||
      testFiles (builtins.tail files) relPath
    );
in
{
  cleanSourceWithFiles = { src, files }:
    pkgs.lib.cleanSourceWith {
      inherit src;
      filter = path: type:
        let
          relPath = builtins.replaceStrings [ (toString src + "/") ] [ "" ] (toString path);
        in
        testFiles files relPath;
    };
}
