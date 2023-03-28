#[doc = "Register `FMC_CSQCFGR2` reader"]
pub struct R(crate::R<FMC_CSQCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_CSQCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_CSQCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_CSQCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_CSQCFGR2` writer"]
pub struct W(crate::W<FMC_CSQCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_CSQCFGR2_SPEC>;
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
impl From<crate::W<FMC_CSQCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_CSQCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQSDTEN` reader - SQSDTEN"]
pub type SQSDTEN_R = crate::BitReader<bool>;
#[doc = "Field `SQSDTEN` writer - SQSDTEN"]
pub type SQSDTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQCFGR2_SPEC, bool, O>;
#[doc = "Field `RCMD2EN` reader - RCMD2EN"]
pub type RCMD2EN_R = crate::BitReader<bool>;
#[doc = "Field `RCMD2EN` writer - RCMD2EN"]
pub type RCMD2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQCFGR2_SPEC, bool, O>;
#[doc = "Field `DMASEN` reader - DMASEN"]
pub type DMASEN_R = crate::BitReader<bool>;
#[doc = "Field `DMASEN` writer - DMASEN"]
pub type DMASEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQCFGR2_SPEC, bool, O>;
#[doc = "Field `RCMD1` reader - RCMD1"]
pub type RCMD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCMD1` writer - RCMD1"]
pub type RCMD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_CSQCFGR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `RCMD2` reader - RCMD2"]
pub type RCMD2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCMD2` writer - RCMD2"]
pub type RCMD2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_CSQCFGR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `RCMD1T` reader - RCMD1T"]
pub type RCMD1T_R = crate::BitReader<bool>;
#[doc = "Field `RCMD1T` writer - RCMD1T"]
pub type RCMD1T_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQCFGR2_SPEC, bool, O>;
#[doc = "Field `RCMD2T` reader - RCMD2T"]
pub type RCMD2T_R = crate::BitReader<bool>;
#[doc = "Field `RCMD2T` writer - RCMD2T"]
pub type RCMD2T_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQCFGR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SQSDTEN"]
    #[inline(always)]
    pub fn sqsdten(&self) -> SQSDTEN_R {
        SQSDTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RCMD2EN"]
    #[inline(always)]
    pub fn rcmd2en(&self) -> RCMD2EN_R {
        RCMD2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMASEN"]
    #[inline(always)]
    pub fn dmasen(&self) -> DMASEN_R {
        DMASEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - RCMD1"]
    #[inline(always)]
    pub fn rcmd1(&self) -> RCMD1_R {
        RCMD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RCMD2"]
    #[inline(always)]
    pub fn rcmd2(&self) -> RCMD2_R {
        RCMD2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - RCMD1T"]
    #[inline(always)]
    pub fn rcmd1t(&self) -> RCMD1T_R {
        RCMD1T_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RCMD2T"]
    #[inline(always)]
    pub fn rcmd2t(&self) -> RCMD2T_R {
        RCMD2T_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SQSDTEN"]
    #[inline(always)]
    #[must_use]
    pub fn sqsdten(&mut self) -> SQSDTEN_W<0> {
        SQSDTEN_W::new(self)
    }
    #[doc = "Bit 1 - RCMD2EN"]
    #[inline(always)]
    #[must_use]
    pub fn rcmd2en(&mut self) -> RCMD2EN_W<1> {
        RCMD2EN_W::new(self)
    }
    #[doc = "Bit 2 - DMASEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmasen(&mut self) -> DMASEN_W<2> {
        DMASEN_W::new(self)
    }
    #[doc = "Bits 8:15 - RCMD1"]
    #[inline(always)]
    #[must_use]
    pub fn rcmd1(&mut self) -> RCMD1_W<8> {
        RCMD1_W::new(self)
    }
    #[doc = "Bits 16:23 - RCMD2"]
    #[inline(always)]
    #[must_use]
    pub fn rcmd2(&mut self) -> RCMD2_W<16> {
        RCMD2_W::new(self)
    }
    #[doc = "Bit 24 - RCMD1T"]
    #[inline(always)]
    #[must_use]
    pub fn rcmd1t(&mut self) -> RCMD1T_W<24> {
        RCMD1T_W::new(self)
    }
    #[doc = "Bit 25 - RCMD2T"]
    #[inline(always)]
    #[must_use]
    pub fn rcmd2t(&mut self) -> RCMD2T_W<25> {
        RCMD2T_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to configure the command sequencer to issue random read/ write commands to read/ write data by sector and automatically read/write data from NAND Flash memory at a programmable address offset. This is useful when performing a sector read/write operation followed by an ECC read/write operation in the NAND Flash spare area.The command sequencer generates the random commands untill all the sectors are read/written. .\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqcfgr2](index.html) module"]
pub struct FMC_CSQCFGR2_SPEC;
impl crate::RegisterSpec for FMC_CSQCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_csqcfgr2::R](R) reader structure"]
impl crate::Readable for FMC_CSQCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_csqcfgr2::W](W) writer structure"]
impl crate::Writable for FMC_CSQCFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_CSQCFGR2 to value 0"]
impl crate::Resettable for FMC_CSQCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
