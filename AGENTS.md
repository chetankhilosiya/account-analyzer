# Account Analyzer - AI Agent Instructions

## Overview
This is a Rust-based application for analyzing bank statements and extracting data based on user queries. It provides a desktop application interface using Dioxus with web capabilities.

## Tech Stack

### Core Language
- **Rust 1.95.0** - Memory-safe systems programming language

### Frameworks and Libraries
- **Dioxus 0.7.10** - Cross-platform UI framework with router support
- **calamine 0.35.0** - Excel file parsing library with chrono support
- **anyhow 1.0.104** - error handling library
- **chrono 0.4.45** - Date and time handling library with serde support
- **dioxus-primitives** - Dioxus component primitives (from GitHub repo)
- **dioxus-tabular** - Table rendering components for Dioxus

### Build Features
- **default** - Enables desktop feature
- **web** - Enables web build target features (requires dioxus/web)
- **desktop** - Enables desktop build target features (requires dioxus/desktop)
- **mobile** - Enables mobile build target features (requires dioxus/mobile)

## Application Purpose
The application analyzes bank statements (likely Excel files) and allows users to query specific data points from their financial records.

## Key Capabilities
1. Parse and analyze bank statement files
2. Extract structured financial data
3. Provide query-based data retrieval
4. Display

## Bank Statement Format Examples

### ICICI Bank Statement Format
The application supports ICICI bank statements with the following column structure:

| Column Name | Description |
|-------------|-------------|
| S No. | Serial number of the transaction |
| Value Date | Date when the transaction is posted to the account |
| Transaction Date | Date when the transaction occurred |

| Transaction Remarks | Description or remarks about the transaction |
| Withdrawal Amount (INR) | Amount debited from the account in Indian Rupees |
| Deposit Amount (INR) | Amount credited to the account in Indian Rupees |
| Balance (INR) | Account balance after the transaction in Indian Rupees |

Example data row:
| 1	| 05/04/2025 | 05/04/2025 | ACH/RACPC SHANKARSHETH R/ICIC0000000004713089/NODN | 24973.00 | 0.0 | 35537.61
| 2 | 05/04/2025 | 05/04/2025 | ACH/RACPC - II WAKDEWADI/ICIC0000000007630007/NODN | 14934.00 | 0.0 | 20603.61

