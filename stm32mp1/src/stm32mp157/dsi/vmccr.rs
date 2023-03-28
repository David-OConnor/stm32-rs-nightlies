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
#[doc = "Field `VMT` reader - VMT"]
pub type VMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPVSAE` reader - LPVSAE"]
pub type LPVSAE_R = crate::BitReader<bool>;
#[doc = "Field `LPVBPE` reader - LPVBPE"]
pub type LPVBPE_R = crate::BitReader<bool>;
#[doc = "Field `LPVFPE` reader - LPVFPE"]
pub type LPVFPE_R = crate::BitReader<bool>;
#[doc = "Field `LPVAE` reader - LPVAE"]
pub type LPVAE_R = crate::BitReader<bool>;
#[doc = "Field `LPHBPE` reader - LPHBPE"]
pub type LPHBPE_R = crate::BitReader<bool>;
#[doc = "Field `LPHFE` reader - LPHFE"]
pub type LPHFE_R = crate::BitReader<bool>;
#[doc = "Field `FBTAAE` reader - FBTAAE"]
pub type FBTAAE_R = crate::BitReader<bool>;
#[doc = "Field `LPCE` reader - LPCE"]
pub type LPCE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - VMT"]
    #[inline(always)]
    pub fn vmt(&self) -> VMT_R {
        VMT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - LPVSAE"]
    #[inline(always)]
    pub fn lpvsae(&self) -> LPVSAE_R {
        LPVSAE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPVBPE"]
    #[inline(always)]
    pub fn lpvbpe(&self) -> LPVBPE_R {
        LPVBPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LPVFPE"]
    #[inline(always)]
    pub fn lpvfpe(&self) -> LPVFPE_R {
        LPVFPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LPVAE"]
    #[inline(always)]
    pub fn lpvae(&self) -> LPVAE_R {
        LPVAE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LPHBPE"]
    #[inline(always)]
    pub fn lphbpe(&self) -> LPHBPE_R {
        LPHBPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LPHFE"]
    #[inline(always)]
    pub fn lphfe(&self) -> LPHFE_R {
        LPHFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FBTAAE"]
    #[inline(always)]
    pub fn fbtaae(&self) -> FBTAAE_R {
        FBTAAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LPCE"]
    #[inline(always)]
    pub fn lpce(&self) -> LPCE_R {
        LPCE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "DSI Host video mode current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vmccr](index.html) module"]
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
