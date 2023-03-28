#[doc = "Register `WRP2R_PRG` reader"]
pub struct R(crate::R<WRP2R_PRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRP2R_PRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRP2R_PRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRP2R_PRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRP2R_PRG` writer"]
pub struct W(crate::W<WRP2R_PRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRP2R_PRG_SPEC>;
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
impl From<crate::W<WRP2R_PRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRP2R_PRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRPSG2` reader - Bank 2 sector group protection option status byte Setting WRPSGn2 bits to 0 write protects the corresponding group of four consecutive sectors in Bank 2 (0: group is write protected; 1: group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127"]
pub type WRPSG2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WRPSG2` writer - Bank 2 sector group protection option status byte Setting WRPSGn2 bits to 0 write protects the corresponding group of four consecutive sectors in Bank 2 (0: group is write protected; 1: group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127"]
pub type WRPSG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRP2R_PRG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Bank 2 sector group protection option status byte Setting WRPSGn2 bits to 0 write protects the corresponding group of four consecutive sectors in Bank 2 (0: group is write protected; 1: group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127"]
    #[inline(always)]
    pub fn wrpsg2(&self) -> WRPSG2_R {
        WRPSG2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bank 2 sector group protection option status byte Setting WRPSGn2 bits to 0 write protects the corresponding group of four consecutive sectors in Bank 2 (0: group is write protected; 1: group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127"]
    #[inline(always)]
    #[must_use]
    pub fn wrpsg2(&mut self) -> WRPSG2_W<0> {
        WRPSG2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH write sector group protection for Bank 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrp2r_prg](index.html) module"]
pub struct WRP2R_PRG_SPEC;
impl crate::RegisterSpec for WRP2R_PRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrp2r_prg::R](R) reader structure"]
impl crate::Readable for WRP2R_PRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrp2r_prg::W](W) writer structure"]
impl crate::Writable for WRP2R_PRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRP2R_PRG to value 0"]
impl crate::Resettable for WRP2R_PRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
