local M = {}

M.defaults = {
    window_width = 80,
    window_height = 80,
    fail_fast = false,
}

---@param options unknown
function M.set(options)
    return vim.tbl_deep_extend("force", {}, M.defaults, options or {})
end

return M
