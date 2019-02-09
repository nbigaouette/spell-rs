from spell._native import ffi, lib

# https://michaeljung.pro/2017/11/12/passing-strings-from-python-to-rust/


def to_cstring(text):
    return ffi.new("char[]", text.encode("utf-8"))

class LcsObject:
    def __init__(self, object):
        self.object = object

    def tokens(self):
        length = lib.object_tokens_len(self.object)

        for i in range(length):
            ptr = lib.object_ith_token(self.object, i)

            yield ffi.string(ptr).decode('utf-8')

    def line_ids(self):
        length = lib.object_lines_ids_len(self.object)

        for i in range(length):
            yield lib.object_ith_line_id(self.object, i)

    def __del__(self):
        lib.free_object(self.object)
        self.object = ffi.NULL

class Spell:
    def __init__(self):
        # FIXME: Take delimiters as argument and store them
        self.map = lib.new_map()

    def insert(self, line):
        lib.insert_in_map(self.map, to_cstring(line))

    def match(self, line):
        return LcsObject(lib.get_match(self.map, to_cstring(line)))

    def __del__(self):
        lib.free_map(self.map)
        self.map = ffi.NULL

