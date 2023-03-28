#[doc = "Register `FMC_CSQCFGR3` reader"]
pub struct R(crate::R<FMC_CSQCFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_CSQCFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_CSQCFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_CSQCFGR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_CSQCFGR3` writer"]
pub struct W(crate::W<FMC_CSQCFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_CSQCFGR3_SPEC>;
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
impl From<crate::W<FMC_CSQCFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_CSQCFGR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SNBR` reader - SNBR"]
pub type SNBR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SNBR` writer - SNBR"]
pub type SNBR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_CSQCFGR3_SPEC, u8, u8, 6, O>;
#[doc = "Field `AC1T` reader - AC1T"]
pub type AC1T_R = crate::BitReader<bool>;
#[doc = "Field `AC1T` writer - AC1T"]
pub type AC1T_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQCFGR3_SPEC, bool, O>;
#[doc = "Field `AC2T` reader - AC2T"]
pub type AC2T_R = crate::BitReader<bool>;
#[doc = "Field `AC2T` writer - AC2T"]
pub type AC2T_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQCFGR3_SPEC, bool, O>;
#[doc = "Field `AC3T` reader - AC3T"]
pub type AC3T_R = crate::BitReader<bool>;
#[doc = "Field `AC3T` writer - AC3T"]
pub type AC3T_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQCFGR3_SPEC, bool, O>;
#[doc = "Field `AC4T` reader - AC4T"]
pub type AC4T_R = crate::BitReader<bool>;
#[doc = "Field `AC4T` writer - AC4T"]
pub type AC4T_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQCFGR3_SPEC, bool, O>;
#[doc = "Field `AC5T` reader - AC5T"]
pub type AC5T_R = crate::BitReader<bool>;
#[doc = "Field `AC5T` writer - AC5T"]
pub type AC5T_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQCFGR3_SPEC, bool, O>;
#[doc = "Field `SDT` reader - SDT"]
pub type SDT_R = crate::BitReader<bool>;
#[doc = "Field `SDT` writer - SDT"]
pub type SDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQCFGR3_SPEC, bool, O>;
#[doc = "Field `RAC1T` reader - RAC1T"]
pub type RAC1T_R = crate::BitReader<bool>;
#[doc = "Field `RAC1T` writer - RAC1T"]
pub type RAC1T_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQCFGR3_SPEC, bool, O>;
#[doc = "Field `RAC2T` reader - RAC2T"]
pub type RAC2T_R = crate::BitReader<bool>;
#[doc = "Field `RAC2T` writer - RAC2T"]
pub type RAC2T_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQCFGR3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 8:13 - SNBR"]
    #[inline(always)]
    pub fn snbr(&self) -> SNBR_R {
        SNBR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - AC1T"]
    #[inline(always)]
    pub fn ac1t(&self) -> AC1T_R {
        AC1T_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AC2T"]
    #[inline(always)]
    pub fn ac2t(&self) -> AC2T_R {
        AC2T_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - AC3T"]
    #[inline(always)]
    pub fn ac3t(&self) -> AC3T_R {
        AC3T_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - AC4T"]
    #[inline(always)]
    pub fn ac4t(&self) -> AC4T_R {
        AC4T_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - AC5T"]
    #[inline(always)]
    pub fn ac5t(&self) -> AC5T_R {
        AC5T_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SDT"]
    #[inline(always)]
    pub fn sdt(&self) -> SDT_R {
        SDT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - RAC1T"]
    #[inline(always)]
    pub fn rac1t(&self) -> RAC1T_R {
        RAC1T_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - RAC2T"]
    #[inline(always)]
    pub fn rac2t(&self) -> RAC2T_R {
        RAC2T_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:13 - SNBR"]
    #[inline(always)]
    #[must_use]
    pub fn snbr(&mut self) -> SNBR_W<8> {
        SNBR_W::new(self)
    }
    #[doc = "Bit 16 - AC1T"]
    #[inline(always)]
    #[must_use]
    pub fn ac1t(&mut self) -> AC1T_W<16> {
        AC1T_W::new(self)
    }
    #[doc = "Bit 17 - AC2T"]
    #[inline(always)]
    #[must_use]
    pub fn ac2t(&mut self) -> AC2T_W<17> {
        AC2T_W::new(self)
    }
    #[doc = "Bit 18 - AC3T"]
    #[inline(always)]
    #[must_use]
    pub fn ac3t(&mut self) -> AC3T_W<18> {
        AC3T_W::new(self)
    }
    #[doc = "Bit 19 - AC4T"]
    #[inline(always)]
    #[must_use]
    pub fn ac4t(&mut self) -> AC4T_W<19> {
        AC4T_W::new(self)
    }
    #[doc = "Bit 20 - AC5T"]
    #[inline(always)]
    #[must_use]
    pub fn ac5t(&mut self) -> AC5T_W<20> {
        AC5T_W::new(self)
    }
    #[doc = "Bit 21 - SDT"]
    #[inline(always)]
    #[must_use]
    pub fn sdt(&mut self) -> SDT_W<21> {
        SDT_W::new(self)
    }
    #[doc = "Bit 22 - RAC1T"]
    #[inline(always)]
    #[must_use]
    pub fn rac1t(&mut self) -> RAC1T_W<22> {
        RAC1T_W::new(self)
    }
    #[doc = "Bit 23 - RAC2T"]
    #[inline(always)]
    #[must_use]
    pub fn rac2t(&mut self) -> RAC2T_W<23> {
        RAC2T_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC NAND sequencer configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqcfgr3](index.html) module"]
pub struct FMC_CSQCFGR3_SPEC;
impl crate::RegisterSpec for FMC_CSQCFGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_csqcfgr3::R](R) reader structure"]
impl crate::Readable for FMC_CSQCFGR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_csqcfgr3::W](W) writer structure"]
impl crate::Writable for FMC_CSQCFGR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_CSQCFGR3 to value 0"]
impl crate::Resettable for FMC_CSQCFGR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
