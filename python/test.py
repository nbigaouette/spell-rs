import spell

spell.lib.init_env_logger()

spell_map = spell.Spell()

spell_map.insert("Command Failed on: node-127,node-234")
spell_map.insert("Command Failed on: node-128,node-234")
spell_map.insert("Command Failed on: node-129,node-235")

lcs_object = spell_map.match("Command Failed on: node-130,node-235")
for i, token in enumerate(lcs_object.tokens()):
    print("%d - Token: %s" % (i, token))
for i, line_id in enumerate(lcs_object.line_ids()):
    print("%d - Line id: %s" % (i, line_id))
