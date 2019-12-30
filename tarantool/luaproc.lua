local log = require('log')

return function(someField)
    log.error('%s', someField)
end
