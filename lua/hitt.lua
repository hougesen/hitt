local M = {}

---@param path string
---@return string | nil
local function send_request(path)
    local cmd = string.format("hitt run --vim %s", path)

    local status, handle, error_msg = pcall(io.popen, cmd)

    if status == false then
        return nil
    end

    if handle == nil then
        return nil
    end

    if error_msg ~= nil then
        return nil
    end

    local file = handle:read("*a")

    handle:close()

    return file
end

local function calculate_window_size()
    local total_width = vim.api.nvim_win_get_width(0)
    local total_height = vim.api.nvim_win_get_height(0)

    local window_width = math.floor(total_width * 0.80)
    local window_height = math.floor(total_height * 0.80)

    local row = math.floor((total_height - window_height) / 2)
    local col = math.floor((total_width - window_width) / 2)

    return {
        row,
        col,
        width = window_width,
        height = window_height,
    }
end

---@param content string
local function show_response(content)
    local buf = vim.api.nvim_create_buf(false, true)

    -- https://github.com/m00qek/baleia.nvim/blob/main/lua/baleia/ansi.lua
    local ansi_pattern = "\x1b[[0-9][:;0-9]*m"

    local lines = {}
    for line in content:gmatch("([^\n]*)\n?") do
        local stripped_line = line:gsub(ansi_pattern, "")

        table.insert(lines, stripped_line)
    end

    vim.api.nvim_buf_set_lines(buf, 0, -1, false, lines)

    local total_width = vim.api.nvim_win_get_width(0)
    local total_height = vim.api.nvim_win_get_height(0)

    local window_width = math.floor(total_width * 0.80)
    local window_height = math.floor(total_height / 2)

    local row = math.floor((total_height - window_height) / 2)
    local col = math.floor((total_width - window_width) / 2)

    vim.api.nvim_open_win(buf, true, {
        relative = "win",
        row = row,
        col = col,
        width = window_width,
        height = window_height,
        border = "rounded",
        title = "hitt",
    })
end

local function get_current_buf_path()
    return vim.api.nvim_buf_get_name(0)
end

function M.HittSendRequest()
    local path = get_current_buf_path()

    if path == nil or path == "" then
        return
    end

    local response = send_request(path)

    if response == nil then
        return
    end

    show_response(response)
end

function M.setup()
    vim.keymap.set("n", "<leader>rr", M.HittSendRequest, {})
end

return M
