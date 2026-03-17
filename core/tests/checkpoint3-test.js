const PrivacyRouter = require('../src/privacy-router');

async function runCheckpoint3Test() {
  const router = new PrivacyRouter();

  console.log("=== Checkpoint 3: Privacy Router Test ===");

  // 1. Test Office Sensitivity (Keyword Match)
  console.log("\n[Test 1] Office Sensitivity (Keywords)");
  const p1 = "What is the SALARY for our new internal manager?";
  const r1 = await router.route(p1);
  console.log(`Prompt: "${p1}"`);
  console.log(`Destination: ${r1.destination} (Reason: ${r1.reason})`);
  if (r1.destination !== 'local') throw new Error("FAIL: Salary should stay local!");

  // 2. Test PII Filtering (SSN Pattern)
  console.log("\n[Test 2] PII Filtering (SSN Pattern)");
  const p2 = "Update record for employee with SSN 123-45-6789";
  const r2 = await router.route(p2);
  console.log(`Prompt: "${p2}"`);
  console.log(`Destination: ${r2.destination} (Reason: ${r2.reason})`);
  if (r2.destination !== 'local') throw new Error("FAIL: SSN should stay local!");

  // 3. Test Cloud Route with Scrubbing (Email Scrub)
  console.log("\n[Test 3] Cloud Route with Scrubbing");
  const p3 = "Draft a public invite for John at john.doe@example.com for our open house.";
  const r3 = await router.route(p3);
  console.log(`Prompt: "${p3}"`);
  console.log(`Destination: ${r3.destination}`);
  console.log(`Scrubbed Payload: "${r3.payload}"`);
  if (r3.destination !== 'cloud') throw new Error("FAIL: Non-sensitive task should go to cloud!");
  if (r3.payload.includes("john.doe@example.com")) throw new Error("FAIL: Email not scrubbed!");

  // 4. Test Pure Public Data
  console.log("\n[Test 4] Pure Public Data");
  const p4 = "Summarize the history of the Apple M2 Max chip.";
  const r4 = await router.route(p4);
  console.log(`Prompt: "${p4}"`);
  console.log(`Destination: ${r4.destination}`);
  if (r4.destination !== 'cloud') throw new Error("FAIL: Public info should go to cloud!");

  console.log("\n=== Checkpoint 3: SUCCESS ===");
}

runCheckpoint3Test().catch(err => {
  console.error("\n=== Checkpoint 3: FAILED ===");
  console.error(err.message);
});
