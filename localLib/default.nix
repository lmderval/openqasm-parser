{ pkgs }:
{
  filters = import ./filters.nix { inherit pkgs; };
}
