local install_path = vim.fn.stdpath("data") .. "/site/pack/packer/start/packer.nvim"
if vim.fn.empty(vim.fn.glob(install_path)) > 0 then
	packer_bootstrap =
		vim.fn.system({ "git", "clone", "--depth", "1", "https://github.com/wbthomason/packer.nvim", install_path })
	vim.cmd([[packadd packer.nvim]])
end

require("packer").startup(function(use)
	use("wbthomason/packer.nvim")

	use({
		"ellisonleao/gruvbox.nvim",
		requires = "rktjmp/lush.nvim",
		config = function()
			require("plugins.gruvbox")
		end,
	})

	use({
		"neovim/nvim-lspconfig",
		requires = { "williamboman/mason.nvim", "williamboman/mason-lspconfig.nvim", "simrat39/rust-tools.nvim" },
		config = function()
			require("plugins.lspconfig")
		end,
	})

	use({
		"nvim-treesitter/nvim-treesitter",
		requires = "p00f/nvim-ts-rainbow",
		run = ":TSUpdate",
		config = function()
			require("plugins.treesitter")
		end,
	})

	use({
		"ms-jpq/coq_nvim",
		branch = "coq",
		requires = {
			{ "ms-jpq/coq.artifacts", branch = "artifacts" },
		},
		config = function()
			require("plugins.coq")
		end,
	})

	use({
		"windwp/nvim-autopairs",
		config = function()
			require("plugins.autopairs")
		end,
	})

	use({
		"akinsho/bufferline.nvim",
		requires = "kyazdani42/nvim-web-devicons",
		config = function()
			require("plugins.bufferline")
		end,
	})

	use({
		"kyazdani42/nvim-tree.lua",
		requires = "kyazdani42/nvim-web-devicons",
		config = function()
			require("plugins.tree")
		end,
	})

	use({
		"famiu/feline.nvim",
		config = function()
			require("feline").setup()
		end,
	})

	use({
		"lewis6991/gitsigns.nvim",
		requires = "nvim-lua/plenary.nvim",
		config = function()
			require("gitsigns").setup()
		end,
	})

	use({
		"norcalli/nvim-colorizer.lua",
		config = function()
			require("colorizer").setup()
		end,
	})

	use({
		"sbdchd/neoformat",
		config = function()
			require("plugins.neoformat")
		end,
	})

	use("matze/vim-move")
	use("farmergreg/vim-lastplace")
	use("haya14busa/is.vim")
	-- use("tyru/caw.vim")
	use({
		"numToStr/Comment.nvim",
		config = function()
			require("Comment").setup()
		end,
	})

	use({
		"phaazon/hop.nvim",
		config = function()
			require("plugins.hop")
		end,
	})

	use({
		"nvim-telescope/telescope.nvim",
		requires = {
			"nvim-lua/plenary.nvim",
			"nvim-telescope/telescope-ui-select.nvim",
			"natecraddock/telescope-zf-native.nvim",
		},
		config = function()
			require("plugins.telescope")
		end,
	})

	use({
		"ray-x/lsp_signature.nvim",
		config = function()
			require("plugins.lsp_signature")
		end,
	})

	use({
		"saecki/crates.nvim",
		event = { "BufRead Cargo.toml" },
		requires = { "nvim-lua/plenary.nvim" },
		config = function()
			require("crates").setup()
		end,
	})
end)
