#[doc = "Register `FMC_SR` reader"]
pub struct R(crate::R<FMC_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_SR` writer"]
pub struct W(crate::W<FMC_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_SR_SPEC>;
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
impl From<crate::W<FMC_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRS` reader - Interrupt rising edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it is set."]
pub type IRS_R = crate::BitReader<bool>;
#[doc = "Field `IRS` writer - Interrupt rising edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it is set."]
pub type IRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_SR_SPEC, bool, O>;
#[doc = "Field `ILS` reader - Interrupt high-level status The flag is set by hardware and reset by software."]
pub type ILS_R = crate::BitReader<bool>;
#[doc = "Field `ILS` writer - Interrupt high-level status The flag is set by hardware and reset by software."]
pub type ILS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_SR_SPEC, bool, O>;
#[doc = "Field `IFS` reader - Interrupt falling edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it is set."]
pub type IFS_R = crate::BitReader<bool>;
#[doc = "Field `IFS` writer - Interrupt falling edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it is set."]
pub type IFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_SR_SPEC, bool, O>;
#[doc = "Field `IREN` reader - Interrupt rising edge detection enable bit"]
pub type IREN_R = crate::BitReader<bool>;
#[doc = "Field `IREN` writer - Interrupt rising edge detection enable bit"]
pub type IREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_SR_SPEC, bool, O>;
#[doc = "Field `ILEN` reader - Interrupt high-level detection enable bit"]
pub type ILEN_R = crate::BitReader<bool>;
#[doc = "Field `ILEN` writer - Interrupt high-level detection enable bit"]
pub type ILEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_SR_SPEC, bool, O>;
#[doc = "Field `IFEN` reader - Interrupt falling edge detection enable bit"]
pub type IFEN_R = crate::BitReader<bool>;
#[doc = "Field `IFEN` writer - Interrupt falling edge detection enable bit"]
pub type IFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_SR_SPEC, bool, O>;
#[doc = "Field `FEMPT` reader - FIFO empty Read-only bit that provides the status of the FIFO"]
pub type FEMPT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Interrupt rising edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it is set."]
    #[inline(always)]
    pub fn irs(&self) -> IRS_R {
        IRS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt high-level status The flag is set by hardware and reset by software."]
    #[inline(always)]
    pub fn ils(&self) -> ILS_R {
        ILS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt falling edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it is set."]
    #[inline(always)]
    pub fn ifs(&self) -> IFS_R {
        IFS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt rising edge detection enable bit"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt high-level detection enable bit"]
    #[inline(always)]
    pub fn ilen(&self) -> ILEN_R {
        ILEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt falling edge detection enable bit"]
    #[inline(always)]
    pub fn ifen(&self) -> IFEN_R {
        IFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO empty Read-only bit that provides the status of the FIFO"]
    #[inline(always)]
    pub fn fempt(&self) -> FEMPT_R {
        FEMPT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt rising edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it is set."]
    #[inline(always)]
    #[must_use]
    pub fn irs(&mut self) -> IRS_W<0> {
        IRS_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt high-level status The flag is set by hardware and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ils(&mut self) -> ILS_W<1> {
        ILS_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt falling edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it is set."]
    #[inline(always)]
    #[must_use]
    pub fn ifs(&mut self) -> IFS_W<2> {
        IFS_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt rising edge detection enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<3> {
        IREN_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt high-level detection enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ilen(&mut self) -> ILEN_W<4> {
        ILEN_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt falling edge detection enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ifen(&mut self) -> IFEN_W<5> {
        IFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO status and interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_sr](index.html) module"]
pub struct FMC_SR_SPEC;
impl crate::RegisterSpec for FMC_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_sr::R](R) reader structure"]
impl crate::Readable for FMC_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_sr::W](W) writer structure"]
impl crate::Writable for FMC_SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_SR to value 0x40"]
impl crate::Resettable for FMC_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
