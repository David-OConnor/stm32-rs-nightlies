#[doc = "Register `CFGR2` reader"]
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR2` writer"]
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLL` reader - Core Lockup Lock"]
pub type CLL_R = crate::BitReader<bool>;
#[doc = "Field `CLL` writer - Core Lockup Lock"]
pub type CLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `SPL` reader - SRAM Parity Lock"]
pub type SPL_R = crate::BitReader<bool>;
#[doc = "Field `SPL` writer - SRAM Parity Lock"]
pub type SPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `PVDL` reader - PVD Lock"]
pub type PVDL_R = crate::BitReader<bool>;
#[doc = "Field `PVDL` writer - PVD Lock"]
pub type PVDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `ECCL` reader - ECC Lock"]
pub type ECCL_R = crate::BitReader<bool>;
#[doc = "Field `ECCL` writer - ECC Lock"]
pub type ECCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `SPF` reader - SRAM Parity Flag"]
pub type SPF_R = crate::BitReader<bool>;
#[doc = "Field `SPF` writer - SRAM Parity Flag"]
pub type SPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Core Lockup Lock"]
    #[inline(always)]
    pub fn cll(&self) -> CLL_R {
        CLL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM Parity Lock"]
    #[inline(always)]
    pub fn spl(&self) -> SPL_R {
        SPL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVD Lock"]
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECC Lock"]
    #[inline(always)]
    pub fn eccl(&self) -> ECCL_R {
        ECCL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM Parity Flag"]
    #[inline(always)]
    pub fn spf(&self) -> SPF_R {
        SPF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core Lockup Lock"]
    #[inline(always)]
    #[must_use]
    pub fn cll(&mut self) -> CLL_W<0> {
        CLL_W::new(self)
    }
    #[doc = "Bit 1 - SRAM Parity Lock"]
    #[inline(always)]
    #[must_use]
    pub fn spl(&mut self) -> SPL_W<1> {
        SPL_W::new(self)
    }
    #[doc = "Bit 2 - PVD Lock"]
    #[inline(always)]
    #[must_use]
    pub fn pvdl(&mut self) -> PVDL_W<2> {
        PVDL_W::new(self)
    }
    #[doc = "Bit 3 - ECC Lock"]
    #[inline(always)]
    #[must_use]
    pub fn eccl(&mut self) -> ECCL_W<3> {
        ECCL_W::new(self)
    }
    #[doc = "Bit 8 - SRAM Parity Flag"]
    #[inline(always)]
    #[must_use]
    pub fn spf(&mut self) -> SPF_W<8> {
        SPF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr2::R](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
