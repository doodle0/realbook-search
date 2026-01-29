# AWS Image Hosting Migration Plan

**Project:** realbook-search
**Document Version:** 1.0
**Date:** 2026-01-30
**Status:** Planning Phase

---

## Executive Summary

This document outlines the plan to migrate Real Book sheet music images from the current Google Drive-based hosting to Amazon Web Services (AWS) infrastructure using S3 and CloudFront.

**Current State:** Images hosted via Google Drive proxy (`wypn9z41ir5bzmgjjalyna.on.drv.tw`)
**Target State:** AWS S3 + CloudFront CDN
**Estimated Timeline:** 2-3 weeks
**Estimated Cost:** $5-15/month (low traffic scenario)

---

## 1. Current Infrastructure Analysis

### Current Hosting Details

**URL Pattern:**
```
https://wypn9z41ir5bzmgjjalyna.on.drv.tw/realbook/rendered/{volume*1000+page}.jpeg
```

**Technical Analysis:**
- **Provider:** Google Drive with nginx proxy layer
- **Image Format:** JPEG
- **Average File Size:** ~700 KB per image
- **Caching:** 7-day cache (604800 seconds)
- **Total Entries:** 1,161 song entries
- **Estimated Total Images:** ~1,200-1,400 pages (accounting for multi-page entries)
- **Total Storage Required:** ~840 MB - 980 MB

**Issues with Current Setup:**
1. **Dependency Risk:** Relies on third-party proxy service that could disappear
2. **No Control:** Cannot optimize images, change formats, or implement custom caching
3. **Performance:** Extra redirect hop (302 → Google Drive thumbnail API)
4. **Reliability:** No SLA or uptime guarantees
5. **Bandwidth Limits:** Google Drive may rate-limit or restrict access
6. **Scalability:** Cannot easily add features (WebP, responsive images, lazy loading)

---

## 2. Proposed AWS Architecture

### Architecture Overview

```
┌─────────────────────────────────────────────────────┐
│                   Users/Browsers                    │
└────────────────────┬────────────────────────────────┘
                     │ HTTPS
                     ▼
┌─────────────────────────────────────────────────────┐
│              CloudFront CDN                         │
│  - Global edge locations                            │
│  - SSL/TLS termination                              │
│  - Cache TTL: 30 days                               │
│  - Origin failover support                          │
└────────────────────┬────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────┐
│              S3 Bucket (Origin)                     │
│  - Bucket: realbook-sheets-production               │
│  - Region: us-east-1 (or closest to users)          │
│  - Storage class: Standard (hot data)               │
│  - Versioning: Enabled                              │
│  - Lifecycle: Archive old versions to Glacier       │
└─────────────────────────────────────────────────────┘
```

### URL Structure

**New URL Pattern:**
```
https://cdn.realbook.example.com/{volume}/{page}.jpeg
# or
https://d1234abcd.cloudfront.net/{volume}/{page}.jpeg
```

**Example:**
- Old: `https://...on.drv.tw/realbook/rendered/1001.jpeg`
- New: `https://cdn.realbook.example.com/1/1.jpeg`

**S3 Key Structure:**
```
s3://realbook-sheets-production/
├── 1/
│   ├── 1.jpeg
│   ├── 2.jpeg
│   └── ...
├── 2/
│   ├── 1.jpeg
│   └── ...
└── 3/
    └── ...
```

---

## 3. Migration Implementation Plan

### Phase 1: Preparation (Week 1)

**Tasks:**
1. ✅ Create AWS account (or use existing)
2. ⬜ Set up IAM roles and policies
   - S3 bucket admin policy
   - CloudFront distribution policy
   - CI/CD user for automated uploads
3. ⬜ Create S3 bucket with appropriate settings
   - Block public access (use CloudFront only)
   - Enable versioning
   - Configure lifecycle rules
   - Set up logging
4. ⬜ Write image download script
   - Fetch all images from current source
   - Verify integrity (check file sizes)
   - Save locally with proper naming

**Script Example:**
```bash
#!/bin/bash
# download_sheets.sh
for vol in 1 2 3; do
  mkdir -p "sheets/$vol"
  # Page ranges per volume (need to determine exact ranges)
  for page in {1..497}; do  # Volume 1 example
    url="https://wypn9z41ir5bzmgjjalyna.on.drv.tw/realbook/rendered/${vol}$(printf "%03d" $page).jpeg"
    curl -L "$url" -o "sheets/$vol/$page.jpeg"
    sleep 0.5  # Rate limiting
  done
done
```

### Phase 2: AWS Setup (Week 1-2)

**S3 Bucket Configuration:**
```json
{
  "Bucket": "realbook-sheets-production",
  "Region": "us-east-1",
  "Versioning": "Enabled",
  "PublicAccess": "Blocked",
  "Encryption": "AES-256",
  "LifecycleRules": [
    {
      "Id": "ArchiveOldVersions",
      "Status": "Enabled",
      "Transitions": [
        {
          "Days": 90,
          "StorageClass": "GLACIER"
        }
      ]
    }
  ]
}
```

**CloudFront Distribution Settings:**
- **Origin:** S3 bucket with OAI (Origin Access Identity)
- **Price Class:** Use only North America and Europe (cheaper)
- **Cache Behavior:**
  - TTL: Min 86400s (1 day), Max 2592000s (30 days), Default 604800s (7 days)
  - Compress: Yes
  - Viewer Protocol: Redirect HTTP to HTTPS
  - Allowed Methods: GET, HEAD, OPTIONS
- **Custom Domain:** cdn.realbook.kro.kr (optional)
- **SSL Certificate:** Free AWS Certificate Manager cert

### Phase 3: Data Migration (Week 2)

**Upload to S3:**
```bash
# Upload all images
aws s3 sync ./sheets/ s3://realbook-sheets-production/ \
  --storage-class STANDARD \
  --content-type "image/jpeg" \
  --cache-control "public, max-age=2592000" \
  --metadata-directive REPLACE

# Verify upload
aws s3 ls s3://realbook-sheets-production/ --recursive | wc -l
```

**Optimization Options (Optional):**
- Convert to WebP for 30-50% size reduction
- Create multiple sizes (thumbnail, medium, full)
- Use AWS Lambda@Edge for on-the-fly transformation

### Phase 4: Backend Integration (Week 2)

**Update Backend Code:**

In `api/src/models.rs`, update `image_url()`:
```rust
impl RealBookEntry {
    pub fn image_url(&self, page: u32) -> String {
        // Old URL
        // format!("https://wypn9z41ir5bzmgjjalyna.on.drv.tw/realbook/rendered/{}.jpeg",
        //         self.volume * 1000 + page)

        // New AWS CloudFront URL
        format!("https://d1234abcd.cloudfront.net/{}/{}.jpeg",
                self.volume, page)
    }
}
```

**Configuration:**
- Store CDN URL in environment variable or config file
- Support fallback to old URL during transition period
- Add URL prefix to application config

### Phase 5: Testing & Rollout (Week 3)

**Testing Checklist:**
- ⬜ Verify all images accessible via CloudFront
- ⬜ Test loading times (should be faster)
- ⬜ Test from multiple geographic locations
- ⬜ Verify SSL certificate works
- ⬜ Test cache behavior (check X-Cache headers)
- ⬜ Load test with multiple concurrent users
- ⬜ Verify fallback to old URLs (if implemented)

**Rollout Strategy:**
1. Deploy backend with new URLs to staging
2. Test thoroughly
3. Deploy to production with feature flag
4. Monitor for 24-48 hours
5. Gradually increase traffic to new CDN
6. Full cutover after validation
7. Keep old URLs as backup for 1 month

---

## 4. Cost Analysis

### AWS Pricing Estimates

**S3 Storage (Standard Class):**
- Storage: 1 GB × $0.023/GB = **$0.023/month**
- PUT requests (one-time): 1,400 × $0.005/1000 = **$0.007**
- GET requests: Negligible (CloudFront caches)

**CloudFront:**
- Data Transfer Out (monthly estimates):
  - 10 GB/month: **$0.85**
  - 50 GB/month: **$4.25**
  - 100 GB/month: **$8.50**
- Requests:
  - 100K requests: **$0.01**
  - 1M requests: **$0.10**

**Route 53 (if using custom domain):**
- Hosted zone: **$0.50/month**
- Queries: ~$0.40/month (1M queries)

### Total Cost Estimates

| Traffic Level | Monthly Cost |
|--------------|--------------|
| **Low** (10 GB, 100K req) | $1.38 |
| **Medium** (50 GB, 500K req) | $5.20 |
| **High** (100 GB, 1M req) | $9.50 |
| **With Custom Domain** | Add $0.90 |

**Compared to:**
- Current: $0 (but unreliable, no control)
- Self-hosting: $5-20+ (VPS/server costs)

**Note:** First 12 months AWS Free Tier includes:
- 5 GB S3 storage
- 20,000 GET requests
- 2,000 PUT requests
- 50 GB CloudFront transfer

---

## 5. Alternative Solutions

### Option A: Continue Current Setup (Status Quo)
**Pros:** Free, already working
**Cons:** No control, reliability risk, performance issues
**Cost:** $0
**Recommendation:** ❌ Not suitable for production

### Option B: AWS S3 + CloudFront (Recommended)
**Pros:** Reliable, fast, scalable, full control
**Cons:** Ongoing cost
**Cost:** $5-10/month
**Recommendation:** ✅ **Best for production**

### Option C: Cloudflare R2 + CDN
**Pros:** No egress fees, cheaper than AWS
**Cons:** Newer service, less mature
**Cost:** $0.015/GB storage, no egress
**Recommendation:** ⚠️ Good alternative, worth considering

### Option D: DigitalOcean Spaces + CDN
**Pros:** Simple pricing, flat $5/month for 250GB
**Cons:** Less global presence than AWS
**Cost:** $5/month flat
**Recommendation:** ⚠️ Good budget option

### Option E: Vercel/Netlify Static Hosting
**Pros:** Free tier available, easy deployment
**Cons:** Not designed for large media files
**Cost:** Free (with limits)
**Recommendation:** ⚠️ Only for small-scale testing

---

## 6. Image Optimization Opportunities

### Current State
- Format: JPEG
- Size: ~700 KB/image
- Compression: Unknown quality level

### Optimization Strategy

**1. Format Conversion:**
- **WebP:** 30-50% smaller, modern browser support
- **AVIF:** 50-70% smaller, cutting-edge (limited support)
- Serve based on `Accept` header

**2. Responsive Images:**
- Thumbnail: 200px width (~20 KB)
- Medium: 800px width (~150 KB)
- Full: Original 1000px (~400 KB)

**3. Progressive Loading:**
- Use LQIP (Low Quality Image Placeholder)
- Lazy load images below the fold

**Implementation:**
```rust
pub fn image_urls(&self, page: u32, size: ImageSize) -> String {
    let base_url = "https://cdn.realbook.example.com";
    match size {
        ImageSize::Thumb => format!("{}/{}/{}-thumb.webp", base_url, self.volume, page),
        ImageSize::Medium => format!("{}/{}/{}-medium.webp", base_url, self.volume, page),
        ImageSize::Full => format!("{}/{}/{}.jpeg", base_url, self.volume, page),
    }
}
```

**Estimated Savings:**
- Storage: ~50% reduction (480 MB instead of 960 MB)
- Bandwidth: ~60% reduction (faster page loads)
- Cost Impact: ~$3-5/month savings at scale

---

## 7. Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| **Migration data loss** | Low | High | Verify checksums, keep backups |
| **Downtime during migration** | Low | Medium | Use blue-green deployment |
| **AWS cost overrun** | Medium | Low | Set up billing alerts, monitor usage |
| **Image quality degradation** | Low | Medium | Test conversions, keep originals |
| **CloudFront cache issues** | Low | Medium | Proper cache headers, invalidation API |
| **Broken links after migration** | Medium | High | Comprehensive testing, fallback URLs |
| **Copyright/legal issues** | Low | High | Ensure proper rights to host images |

---

## 8. Success Metrics

**Performance:**
- ✅ Reduce image load time by 30%+
- ✅ Achieve 99.9% uptime
- ✅ Cache hit rate >90% on CloudFront

**Cost:**
- ✅ Stay under $10/month for first year
- ✅ No unexpected billing spikes

**User Experience:**
- ✅ Faster page loads (measure via Web Vitals)
- ✅ No broken images
- ✅ Consistent experience across regions

---

## 9. Implementation Checklist

### Pre-Migration
- [ ] Obtain AWS account
- [ ] Verify budget approval ($10/month)
- [ ] Confirm legal rights to host images
- [ ] Set up development/staging environment
- [ ] Document current image URLs for rollback

### Migration
- [ ] Create S3 bucket with proper settings
- [ ] Set up CloudFront distribution
- [ ] Download all images from current source
- [ ] Verify image integrity (checksums)
- [ ] Upload to S3 with correct metadata
- [ ] Configure CloudFront cache behaviors
- [ ] Test image access via CloudFront
- [ ] Update backend URL generation code
- [ ] Deploy to staging environment
- [ ] Run automated tests
- [ ] Perform manual QA testing

### Post-Migration
- [ ] Monitor CloudFront metrics (first 48 hours)
- [ ] Check AWS billing (first week)
- [ ] Verify cache hit rates
- [ ] Collect user feedback
- [ ] Document final architecture
- [ ] Update CLAUDE.md and README.md
- [ ] Archive old image source (backup)
- [ ] Set up automated monitoring alerts

---

## 10. Next Steps

### Immediate Actions (This Week)
1. **Decision:** Approve migration plan and budget
2. **Setup:** Create AWS account or use existing
3. **Research:** Determine exact page counts per volume
4. **Script:** Write image download script

### Week 1
1. Download all images from current source
2. Set up S3 bucket and CloudFront
3. Upload images to S3
4. Test access via CloudFront URLs

### Week 2
1. Update backend code with new URLs
2. Deploy to staging and test
3. Implement fallback mechanism
4. Prepare rollout plan

### Week 3
1. Deploy to production (feature flag)
2. Monitor and validate
3. Full cutover
4. Document changes

---

## 11. References & Resources

**AWS Documentation:**
- [S3 Best Practices](https://docs.aws.amazon.com/AmazonS3/latest/userguide/best-practices.html)
- [CloudFront Developer Guide](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/)
- [AWS Pricing Calculator](https://calculator.aws/)

**Image Optimization:**
- [WebP Format](https://developers.google.com/speed/webp)
- [Responsive Images Guide](https://web.dev/responsive-images/)

**Alternatives:**
- [Cloudflare R2](https://developers.cloudflare.com/r2/)
- [DigitalOcean Spaces](https://www.digitalocean.com/products/spaces)

---

## Appendix A: Cost Breakdown Spreadsheet

| Item | Unit | Quantity | Unit Cost | Monthly Cost |
|------|------|----------|-----------|--------------|
| S3 Storage | GB | 1 | $0.023 | $0.023 |
| S3 PUT Requests | per 1000 | 1.4 | $0.005 | $0.007 |
| CloudFront Transfer | GB | 50 | $0.085 | $4.25 |
| CloudFront Requests | per 10000 | 50 | $0.001 | $0.005 |
| Route 53 (optional) | zone | 1 | $0.50 | $0.50 |
| **Total (without domain)** | | | | **$4.29** |
| **Total (with domain)** | | | | **$4.79** |

---

**Document Prepared By:** Claude Code
**Review Status:** Draft
**Requires Approval:** Yes
