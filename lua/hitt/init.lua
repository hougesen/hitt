local hitt_config = require("hitt.config")

local M = {
    conf = hitt_config.defaults,
}

---@param path string
---@return string | nil
local function send_request(path)
    local cmd = "hitt run --vim"

    if M.conf.fail_fast == true then
        cmd = cmd .. " --fail-fast"
    end

    cmd = cmd .. " " .. path

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

local function get_current_buf_path()
    return vim.api.nvim_buf_get_name(0)
end

---@param content string
---@return string[]
local function split_response_lines(content)
    -- https://github.com/m00qek/baleia.nvim/blob/main/lua/baleia/ansi.lua
    local ansi_pattern = "\x1b[[0-9][:;0-9]*m"
    ---@type string[]
    local lines = {}
    for line in content:gmatch("([^\n]*)\n?") do
        local stripped_line = line:gsub(ansi_pattern, "")

        table.insert(lines, stripped_line)
    end

    return lines
end

---@param content string
local function show_response(content)
    local lines = split_response_lines(content)

    local buf = vim.api.nvim_create_buf(false, true)

    vim.api.nvim_buf_set_lines(buf, 0, -1, false, lines)

    local total_width = vim.api.nvim_win_get_width(0)
    local total_height = vim.api.nvim_win_get_height(0)

    local window_width = math.floor(total_width * (M.conf.window_width / 100))
    local window_height = math.floor(total_height * (M.conf.window_height / 100))

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

    vim.bo.filetype = "http"
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

function M.setup(opts)
    M.conf = hitt_config.set(opts or {})

    vim.api.nvim_create_user_command("HittSendRequest", M.HittSendRequest, { desc = "Send http request using hitt" })
end

return M
