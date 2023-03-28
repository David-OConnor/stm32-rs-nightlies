#[doc = "Register `SBS_MESR` reader"]
pub struct R(crate::R<SBS_MESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBS_MESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBS_MESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBS_MESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SBS_MESR` writer"]
pub struct W(crate::W<SBS_MESR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SBS_MESR_SPEC>;
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
impl From<crate::W<SBS_MESR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SBS_MESR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCLR` reader - erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software"]
pub type MCLR_R = crate::BitReader<bool>;
#[doc = "Field `MCLR` writer - erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software"]
pub type MCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SBS_MESR_SPEC, bool, O>;
#[doc = "Field `IPMEE` reader - end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software."]
pub type IPMEE_R = crate::BitReader<bool>;
#[doc = "Field `IPMEE` writer - end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software."]
pub type IPMEE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SBS_MESR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software"]
    #[inline(always)]
    pub fn mclr(&self) -> MCLR_R {
        MCLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software."]
    #[inline(always)]
    pub fn ipmee(&self) -> IPMEE_R {
        IPMEE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn mclr(&mut self) -> MCLR_W<0> {
        MCLR_W::new(self)
    }
    #[doc = "Bit 16 - end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ipmee(&mut self) -> IPMEE_W<16> {
        IPMEE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SBS memory erase status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbs_mesr](index.html) module"]
pub struct SBS_MESR_SPEC;
impl crate::RegisterSpec for SBS_MESR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sbs_mesr::R](R) reader structure"]
impl crate::Readable for SBS_MESR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sbs_mesr::W](W) writer structure"]
impl crate::Writable for SBS_MESR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SBS_MESR to value 0"]
impl crate::Resettable for SBS_MESR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
