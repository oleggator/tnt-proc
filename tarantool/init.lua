box.cfg{ listen = 3301 }

box.schema.user.grant('guest', 'read,write,execute', 'universe', nil, { if_not_exists = 'true' })

local space = box.schema.space.create('space', { if_not_exists = true })
space:format{
    { 'uuid', 'string'},
    { 'some_str', 'string' },
    { 'another_str', 'string' },
}
space:create_index('primary', { parts = { 1, 'string' }, if_not_exists = true })

-- c function
box.schema.func.create('libcproc.cproc', { language = 'C', if_not_exists = true })
box.schema.user.grant('guest', 'execute', 'function', 'libcproc.cproc', { if_not_exists = true })

-- c function
box.schema.func.create('libcppproc.cppproc', { language = 'C', if_not_exists = true })
box.schema.user.grant('guest', 'execute', 'function', 'libcppproc.cppproc', { if_not_exists = true })

-- rust function
box.schema.func.create('librustproc.rustproc', { language = 'C', if_not_exists = true })
box.schema.user.grant('guest', 'execute', 'function', 'librustproc.rustproc', { if_not_exists = true })

-- lua function
rawset(_G, 'luaproc', require('luaproc'))
