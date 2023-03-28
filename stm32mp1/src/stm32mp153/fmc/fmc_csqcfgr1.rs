#[doc = "Register `FMC_CSQCFGR1` reader"]
pub struct R(crate::R<FMC_CSQCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_CSQCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_CSQCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_CSQCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_CSQCFGR1` writer"]
pub struct W(crate::W<FMC_CSQCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_CSQCFGR1_SPEC>;
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
impl From<crate::W<FMC_CSQCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_CSQCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD2EN` reader - CMD2EN"]
pub type CMD2EN_R = crate::BitReader<bool>;
#[doc = "Field `CMD2EN` writer - CMD2EN"]
pub type CMD2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQCFGR1_SPEC, bool, O>;
#[doc = "Field `DMADEN` reader - DMADEN"]
pub type DMADEN_R = crate::BitReader<bool>;
#[doc = "Field `DMADEN` writer - DMADEN"]
pub type DMADEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQCFGR1_SPEC, bool, O>;
#[doc = "Field `ACYNBR` reader - ACYNBR"]
pub type ACYNBR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACYNBR` writer - ACYNBR"]
pub type ACYNBR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_CSQCFGR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `CMD1` reader - CMD1"]
pub type CMD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMD1` writer - CMD1"]
pub type CMD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_CSQCFGR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `CMD2` reader - CMD2"]
pub type CMD2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMD2` writer - CMD2"]
pub type CMD2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_CSQCFGR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `CMD1T` reader - CMD1T"]
pub type CMD1T_R = crate::BitReader<bool>;
#[doc = "Field `CMD1T` writer - CMD1T"]
pub type CMD1T_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQCFGR1_SPEC, bool, O>;
#[doc = "Field `CMD2T` reader - CMD2T"]
pub type CMD2T_R = crate::BitReader<bool>;
#[doc = "Field `CMD2T` writer - CMD2T"]
pub type CMD2T_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQCFGR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - CMD2EN"]
    #[inline(always)]
    pub fn cmd2en(&self) -> CMD2EN_R {
        CMD2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMADEN"]
    #[inline(always)]
    pub fn dmaden(&self) -> DMADEN_R {
        DMADEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - ACYNBR"]
    #[inline(always)]
    pub fn acynbr(&self) -> ACYNBR_R {
        ACYNBR_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:15 - CMD1"]
    #[inline(always)]
    pub fn cmd1(&self) -> CMD1_R {
        CMD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CMD2"]
    #[inline(always)]
    pub fn cmd2(&self) -> CMD2_R {
        CMD2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - CMD1T"]
    #[inline(always)]
    pub fn cmd1t(&self) -> CMD1T_R {
        CMD1T_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CMD2T"]
    #[inline(always)]
    pub fn cmd2t(&self) -> CMD2T_R {
        CMD2T_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CMD2EN"]
    #[inline(always)]
    #[must_use]
    pub fn cmd2en(&mut self) -> CMD2EN_W<1> {
        CMD2EN_W::new(self)
    }
    #[doc = "Bit 2 - DMADEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmaden(&mut self) -> DMADEN_W<2> {
        DMADEN_W::new(self)
    }
    #[doc = "Bits 4:6 - ACYNBR"]
    #[inline(always)]
    #[must_use]
    pub fn acynbr(&mut self) -> ACYNBR_W<4> {
        ACYNBR_W::new(self)
    }
    #[doc = "Bits 8:15 - CMD1"]
    #[inline(always)]
    #[must_use]
    pub fn cmd1(&mut self) -> CMD1_W<8> {
        CMD1_W::new(self)
    }
    #[doc = "Bits 16:23 - CMD2"]
    #[inline(always)]
    #[must_use]
    pub fn cmd2(&mut self) -> CMD2_W<16> {
        CMD2_W::new(self)
    }
    #[doc = "Bit 24 - CMD1T"]
    #[inline(always)]
    #[must_use]
    pub fn cmd1t(&mut self) -> CMD1T_W<24> {
        CMD1T_W::new(self)
    }
    #[doc = "Bit 25 - CMD2T"]
    #[inline(always)]
    #[must_use]
    pub fn cmd2t(&mut self) -> CMD2T_W<25> {
        CMD2T_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC NAND Command Sequencer Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqcfgr1](index.html) module"]
pub struct FMC_CSQCFGR1_SPEC;
impl crate::RegisterSpec for FMC_CSQCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_csqcfgr1::R](R) reader structure"]
impl crate::Readable for FMC_CSQCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_csqcfgr1::W](W) writer structure"]
impl crate::Writable for FMC_CSQCFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_CSQCFGR1 to value 0"]
impl crate::Resettable for FMC_CSQCFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
