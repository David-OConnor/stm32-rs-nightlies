#[doc = "Register `VMCCR` reader"]
pub struct R(crate::R<VMCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VMCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VMCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VMCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VMT` reader - Video mode Type"]
pub type VMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPVSAE` reader - Low-Power Vertical Sync time Enable"]
pub type LPVSAE_R = crate::BitReader<bool>;
#[doc = "Field `LPVBPE` reader - Low-power Vertical Back-Porch Enable"]
pub type LPVBPE_R = crate::BitReader<bool>;
#[doc = "Field `LPVFPE` reader - Low-power Vertical Front-Porch Enable"]
pub type LPVFPE_R = crate::BitReader<bool>;
#[doc = "Field `LPVAE` reader - Low-Power Vertical Active Enable"]
pub type LPVAE_R = crate::BitReader<bool>;
#[doc = "Field `LPHBPE` reader - Low-power Horizontal Back-Porch Enable"]
pub type LPHBPE_R = crate::BitReader<bool>;
#[doc = "Field `LPHFE` reader - Low-Power Horizontal Front-Porch Enable"]
pub type LPHFE_R = crate::BitReader<bool>;
#[doc = "Field `FBTAAE` reader - Frame BTA Acknowledge Enable"]
pub type FBTAAE_R = crate::BitReader<bool>;
#[doc = "Field `LPCE` reader - Low-Power Command Enable"]
pub type LPCE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - Video mode Type"]
    #[inline(always)]
    pub fn vmt(&self) -> VMT_R {
        VMT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Low-Power Vertical Sync time Enable"]
    #[inline(always)]
    pub fn lpvsae(&self) -> LPVSAE_R {
        LPVSAE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Low-power Vertical Back-Porch Enable"]
    #[inline(always)]
    pub fn lpvbpe(&self) -> LPVBPE_R {
        LPVBPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low-power Vertical Front-Porch Enable"]
    #[inline(always)]
    pub fn lpvfpe(&self) -> LPVFPE_R {
        LPVFPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Low-Power Vertical Active Enable"]
    #[inline(always)]
    pub fn lpvae(&self) -> LPVAE_R {
        LPVAE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Low-power Horizontal Back-Porch Enable"]
    #[inline(always)]
    pub fn lphbpe(&self) -> LPHBPE_R {
        LPHBPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Low-Power Horizontal Front-Porch Enable"]
    #[inline(always)]
    pub fn lphfe(&self) -> LPHFE_R {
        LPHFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Frame BTA Acknowledge Enable"]
    #[inline(always)]
    pub fn fbtaae(&self) -> FBTAAE_R {
        FBTAAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Low-Power Command Enable"]
    #[inline(always)]
    pub fn lpce(&self) -> LPCE_R {
        LPCE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "DSI Host Video mode Current Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vmccr](index.html) module"]
pub struct VMCCR_SPEC;
impl crate::RegisterSpec for VMCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vmccr::R](R) reader structure"]
impl crate::Readable for VMCCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VMCCR to value 0"]
impl crate::Resettable for VMCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
