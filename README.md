# 📌 laravel-config-tracker

Sometimes files under `config` directory of [`laravel/laravel`](https://github.com/laravel/laravel) were changed.
However, we can't be notified.

This utility helps us track the Git changes of those config files.
Once they changed, an issue will be created and sent to [`bs-community/blessing-skin-server`](https://github.com/bs-community/blessing-skin-server).

We use this with GitHub Actions and run this every day.

## 🔌 Usage

Build this utility with Rust, you will get a binary.

Before running it, please specify an environment variable called `GH_TOKEN`.
Set the GitHub token there, because it's used to create issue.

## 📃 License

MIT License

2020-present (c) The Blessing Skin Team
