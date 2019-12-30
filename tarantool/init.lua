box.cfg{listen = 3301}

box.schema.func.create('libcproc.cproc', { language = 'C', if_not_exists = true })
box.schema.user.grant('guest', 'execute', 'function', 'libcproc.cproc', { if_not_exists = true })

box.schema.func.create('librustproc.rustproc', { language = 'C', if_not_exists = true })
box.schema.user.grant('guest', 'execute', 'function', 'librustproc.rustproc', { if_not_exists = true })

rawset(_G, 'luaproc', require('luaproc'))
