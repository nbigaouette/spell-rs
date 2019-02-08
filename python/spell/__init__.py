from spell._native import ffi, lib

# https://michaeljung.pro/2017/11/12/passing-strings-from-python-to-rust/


def to_cstring(text):
    return ffi.new("char[]", text.encode("utf-8"))

class Spell:
    def __init__(self):
        self.map = lib.new_map()

    def insert(self, line):
        lib.insert_in_map(self.map, to_cstring(line))

    def __del__(self):
        lib.free_map(self.map)
        self.map = ffi.NULL


def test():
    print("Initializing logger...")
    lib.init_env_logger()
    print("Initializing logger done.")
    print("Creating map...")
    spell_map = Spell()
    print("Creating map done.")

    print("Inserting line into map...")
    spell_map.insert("Command Failed on: node-127,node-234")
    spell_map.insert("Command Failed on: node-128,node-234")
    spell_map.insert("Command Failed on: node-129,node-235")
    spell_map.insert("Command Failed on: node-130,node-235")

    print("Freeing...")
