async function postJson(url, payload) {
  const res = await fetch(url, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });
  const data = await res.json();
  return { status: res.status, data };
}

function getMode() {
  const checked = document.querySelector('input[name="mode"]:checked');
  return checked ? checked.value : "creative";
}

function renderResponse(status, data) {
  document.getElementById("responseOutput").textContent = JSON.stringify(
    { status, ...data },
    null,
    2
  );
  const sig = data.c0_signature ? data.c0_signature : {};
  document.getElementById("signatureOutput").textContent = Object.keys(sig).length
    ? JSON.stringify(sig, null, 2)
    : "N/A";
}

document.getElementById("submitBtn").addEventListener("click", async () => {
  const mode = getMode();
  const prompt = document.getElementById("prompt").value;
  const mediaRaw = document.getElementById("media").value || "";
  const media = mediaRaw
    .split(",")
    .map((m) => m.trim())
    .filter(Boolean);
  const temperature = parseFloat(document.getElementById("temperature").value) || 0.9;
  const top_k = parseInt(document.getElementById("topK").value, 10) || 64;
  const max_steps = parseInt(document.getElementById("maxSteps").value, 10) || 1024;
  const lightning_invoice = document.getElementById("invoice").value || undefined;
  const free_local = document.getElementById("freeLocal").checked;
  const allow_network = document.getElementById("allowNetwork").checked;
  const axiom_set = document.getElementById("axiomSet").value || "";

  let url = "/api/v1/creative";
  const payload = {
    prompt,
    media,
    temperature,
    top_k,
    lightning_invoice,
  };

  if (mode === "verified") {
    url = "/api/v1/verified";
    Object.assign(payload, {
      axiom_set: axiom_set || '{"name":"default","version":"1","rules":[]}',
      max_steps,
      free_local,
      allow_network,
    });
  }

  try {
    const { status, data } = await postJson(url, payload);
    renderResponse(status, data);
  } catch (err) {
    renderResponse(500, { error: err.message || "request failed" });
  }
});
