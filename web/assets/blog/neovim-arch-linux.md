# My Neovim + Arch Linux Setup: A Minimalist Dev Environment

There's a particular kind of developer who, instead of spinning up a new VM in 30 seconds, spends 6 hours reading the Arch Wiki at 2am getting their initramfs right. I am that developer. This is my setup — not a tutorial, but an explanation of *why* this stack makes me more productive.

## Why Arch Linux?

The Arch philosophy is simple: you get exactly what you put in. No GNOME pre-installed, no NetworkManager enabled by default, no systemd services you didn't ask for. After installation, you have a kernel, a shell, and your own decisions to make.

That process is educational in a way that `apt install ubuntu-desktop` simply isn't. Installing Arch taught me:
- How the Linux boot process actually works (bootloader → kernel → initramfs → init → getty)
- What a filesystem hierarchy is for (why `/etc` vs `/var` vs `/usr/local`)
- Network interfaces, udev rules, kernel modules
- How PAM handles authentication, how D-Bus works, how Wayland compositors negotiate with clients

Beyond education: Arch has the AUR (Arch User Repository), which means bleeding-edge software. The rolling release model means I'm not waiting 2 years for a new kernel or gcc version. For a developer, staying current matters.

## Dotfiles Management

I manage my dotfiles with a bare git repository — no symlink managers, no extra tooling:

```bash
# Initial setup
git init --bare $HOME/.dotfiles
alias config='git --git-dir=$HOME/.dotfiles/ --work-tree=$HOME'
config config status.showUntrackedFiles no

# Track files
config add ~/.config/nvim/init.lua
config commit -m "add neovim config"
config push origin main

# On a new machine
git clone --bare https://github.com/FelipeFTN/dotfiles $HOME/.dotfiles
alias config='git --git-dir=$HOME/.dotfiles/ --work-tree=$HOME'
config checkout
```

This treats `$HOME` as the working tree. Clean, portable, no dependencies.

## Neovim Config Structure

My Neovim config is written entirely in Lua, structured around `lazy.nvim`:

```
~/.config/nvim/
├── init.lua          -- entry point, loads lazy.nvim
├── lua/
│   ├── options.lua   -- vim.opt settings
│   ├── keymaps.lua   -- custom keybindings
│   └── plugins/
│       ├── lsp.lua
│       ├── completion.lua
│       ├── telescope.lua
│       ├── treesitter.lua
│       └── ui.lua
```

```lua
-- init.lua
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not vim.loop.fs_stat(lazypath) then
  vim.fn.system({ "git", "clone", "--filter=blob:none",
    "https://github.com/folke/lazy.nvim.git", lazypath })
end
vim.opt.rtp:prepend(lazypath)

require("lazy").setup("plugins", {
  change_detection = { notify = false },
})
```

## Key Plugins

**nvim-lspconfig + mason.nvim**: LSP client configuration. One-line setup for each language server:
```lua
require("lspconfig").rust_analyzer.setup({
  settings = {
    ["rust-analyzer"] = {
      checkOnSave = { command = "clippy" },
      cargo = { allFeatures = true },
    }
  }
})
require("lspconfig").gopls.setup({})
require("lspconfig").ts_ls.setup({})
```

**nvim-cmp**: Completion engine. Sources: LSP, buffer, path, snippets. The difference between a good completion setup and a bad one is latency — nvim-cmp is fast enough that it never feels like it's in your way.

**telescope.nvim**: Fuzzy finder. `<leader>ff` to find files, `<leader>fg` to grep, `<leader>fb` to switch buffers. With ripgrep as the backend it searches a large codebase instantly.

**nvim-treesitter**: Syntax trees, not regex. Accurate syntax highlighting, smart text objects (`vaf` to select a function, `vic` to select a class), and incremental selection. Once you've used treesitter-aware editing, going back feels broken.

**which-key.nvim**: Shows available keybindings as you type. Essential when you have 80+ custom mappings.

**gitsigns.nvim**: Inline git blame, hunk staging directly in the buffer. Combined with `fugitive` for full git operations, I rarely need to leave Neovim for git work.

## LSP Setup for Rust/Go/TypeScript

The trio I use daily:
- **rust-analyzer**: best-in-class Rust LSP. Inline type hints, macro expansion, clippy integration.
- **gopls**: Go's official language server. Fast, accurate, great struct field completion.
- **ts_ls** (TypeScript Language Server): solid for TypeScript, though I use it less.

Mason handles automatic installation: `:MasonInstall rust-analyzer gopls typescript-language-server`.

## Tiling WM: Hyprland

I run Hyprland (Wayland compositor with tiling). The productivity argument for tiling WMs is simple: your hands never leave the keyboard to manage window positions. Every workflow is a keybinding.

My most-used bindings:
- `SUPER+Return` → terminal
- `SUPER+d` → application launcher (rofi/wofi)
- `SUPER+1..9` → switch workspace
- `SUPER+SHIFT+1..9` → move window to workspace
- `SUPER+h/j/k/l` → focus left/down/up/right (vim-style)

With Hyprland's animation config, workspace switching has a smooth slide — it feels polished without sacrificing speed.

## Terminal: Kitty

Kitty is a GPU-accelerated terminal emulator. It renders via OpenGL, which means zero latency on text rendering even with large scrollback buffers. Font: JetBrains Mono, size 13, with ligatures enabled.

The killer feature: kitty's `kitten` system. `kitty +kitten diff` for side-by-side diffs with syntax highlighting. `kitty +kitten icat` to display images in the terminal. Small things that add up.

## The Productivity Impact

The honest answer: the productivity gain isn't in any single tool. It's in *friction reduction*. When your environment does exactly what you expect, when every operation has a keybinding, when your tools are fast enough to feel instantaneous — you stay in flow longer.

The deeper benefit is understanding. Knowing exactly what's running on your system, why, and how, makes you a better engineer. You stop treating your computer as a black box.

The setup I've described took months to refine. If you're starting out, the ROI might not justify the investment right away. But if you're the kind of engineer who reads kernel source code for fun — Arch + Neovim is waiting for you.
