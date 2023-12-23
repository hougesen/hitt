local M = {}

M.defaults = {
    window_width_percentage = 80,
    window_height_percentage = 80,
}

---@param options unknown
function M.set(options)
    return vim.tbl_deep_extend("force", {}, M.defaults, options or {})
end

return M
