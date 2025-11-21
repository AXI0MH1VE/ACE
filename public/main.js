async function postJson(url, payload) {
  const res = await fetch(url, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });
  return res.json();
}

document.getElementById("runCreative").addEventListener("click", async () => {
  const prompt = document.getElementById("prompt").value;
  const data = await postJson("/api/v1/creative", { prompt, media: ["text"], temperature: 0.9 });
  document.getElementById("creativeOutput").textContent = JSON.stringify(data, null, 2);
});

document.getElementById("runVerified").addEventListener("click", async () => {
  const prompt = document.getElementById("prompt").value;
  const axiom_set = document.getElementById("axioms").value || "default_finance_axioms";
  const allow_network = document.getElementById("allowNetwork").checked;
  const free_local = document.getElementById("freeLocal").checked;
  const payment_token = document.getElementById("paymentToken").value || undefined;
  const data = await postJson("/api/v1/verified", {
    prompt,
    axiom_set,
    max_steps: 2048,
    allow_network,
    free_local,
    payment_token
  });
  document.getElementById("verifiedOutput").textContent = JSON.stringify(data, null, 2);
});
