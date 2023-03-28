#[doc = "Register `WRP1R_PRG` reader"]
pub struct R(crate::R<WRP1R_PRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRP1R_PRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRP1R_PRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRP1R_PRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRP1R_PRG` writer"]
pub struct W(crate::W<WRP1R_PRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRP1R_PRG_SPEC>;
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
impl From<crate::W<WRP1R_PRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRP1R_PRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRPSG1` reader - Bank1 sector group protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127"]
pub type WRPSG1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WRPSG1` writer - Bank1 sector group protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127"]
pub type WRPSG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRP1R_PRG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Bank1 sector group protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127"]
    #[inline(always)]
    pub fn wrpsg1(&self) -> WRPSG1_R {
        WRPSG1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bank1 sector group protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127"]
    #[inline(always)]
    #[must_use]
    pub fn wrpsg1(&mut self) -> WRPSG1_W<0> {
        WRPSG1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH write sector group protection for Bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrp1r_prg](index.html) module"]
pub struct WRP1R_PRG_SPEC;
impl crate::RegisterSpec for WRP1R_PRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrp1r_prg::R](R) reader structure"]
impl crate::Readable for WRP1R_PRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrp1r_prg::W](W) writer structure"]
impl crate::Writable for WRP1R_PRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRP1R_PRG to value 0"]
impl crate::Resettable for WRP1R_PRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
