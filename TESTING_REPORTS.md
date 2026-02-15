# Testing the Inventory Reporting Feature

The sample data has been loaded into your database with 40 items across 5 inventories, but they're not yet assigned to any user. Follow these steps to test the reporting features:

## Step 1: Create Your User Account

1. **Start the application:**
   ```powershell
   $env:DATABASE_URL = "postgres://postgres:password@localhost:5432/home_inventory"
   $env:RUST_LOG = "info"
   .\target\debug\home-registry.exe
   ```

2. **Open your browser** and go to: http://localhost:8210

3. **Create your account** using the first-time setup wizard or registration page

## Step 2: Assign Sample Data to Your Account

Once you've created your account, run this command to assign the sample inventories to your user:

```powershell
.\assign_sample_data.ps1 -Username "YOUR_USERNAME"
```

This will assign all 5 sample inventories (40 items) to your account.

## Step 3: Test the Reporting Feature

Now you can test all the reporting endpoints:

```powershell
.\test_reporting.ps1 -Username "YOUR_USERNAME" -Password "YOUR_PASSWORD"
```

Or run it without parameters and it will prompt you for credentials:

```powershell
.\test_reporting.ps1
```

## What Gets Tested

The test script will verify:

1. **Statistics Endpoint** - Overall inventory statistics (total items, value, categories)
2. **Category Breakdown** - Items grouped by category with percentages
3. **Full Report (JSON)** - Complete inventory report with filtering
4. **Filtered Report** - Testing filters (category, price range, etc.)
5. **CSV Export** - Downloading inventory report as CSV file

## Sample Data Contents

- **Home Office** (8 items): ~$4,431
- **Living Room** (7 items): ~$6,038  
- **Kitchen** (8 items): ~$1,960
- **Garage** (9 items): ~$2,790
- **Master Bedroom** (8 items): ~$4,160

**Total**: 40 items, ~$19,378

## Available API Endpoints

After testing with the script, you can manually test these endpoints (remember to include your JWT token in the Authorization header):

- `GET /api/reports/inventory` - Full report (add `?format=csv` for CSV)
- `GET /api/reports/inventory/statistics` - Statistics only  
- `GET /api/reports/inventory/categories` - Category breakdown

### Filter Options
- `inventory_id` - Filter by specific inventory
- `category` - Filter by category name
- `location` - Search by location (partial match)
- `from_date` - Items purchased after this date (YYYY-MM-DD)
- `to_date` - Items purchased before this date (YYYY-MM-DD)
- `min_price` - Minimum purchase price
- `max_price` - Maximum purchase price
- `sort_by` - Sort by: name, price, date, category
- `sort_order` - asc or desc
- `format` - json or csv

## Troubleshooting

If endpoints return "not found":
- Make sure the server was rebuilt after the import fixes: `cargo build`
- Restart the server to load the new code
- Check that the server started successfully (look for "starting service" in logs)

If you don't see the sample inventories:
- Make sure you ran `.\assign_sample_data.ps1` with your username
- Verify inventories exist: `docker compose exec db psql -U postgres -d home_inventory -c "SELECT * FROM inventories WHERE id BETWEEN 100 AND 104;"`
