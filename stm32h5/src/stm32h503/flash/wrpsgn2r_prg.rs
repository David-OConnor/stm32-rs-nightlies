#[doc = "Register `WRPSGN2R_PRG` reader"]
pub struct R(crate::R<WRPSGN2R_PRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRPSGN2R_PRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRPSGN2R_PRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRPSGN2R_PRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRPSGN2R_PRG` writer"]
pub struct W(crate::W<WRPSGN2R_PRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRPSGN2R_PRG_SPEC>;
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
impl From<crate::W<WRPSGN2R_PRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRPSGN2R_PRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRPSG2` reader - Bank2 sector protection option status byte Setting WRPSG2 bits to 0 write protects the corresponding sectors in bank 2 (0: write protected; 1: not write protected)"]
pub type WRPSG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRPSG2` writer - Bank2 sector protection option status byte Setting WRPSG2 bits to 0 write protects the corresponding sectors in bank 2 (0: write protected; 1: not write protected)"]
pub type WRPSG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRPSGN2R_PRG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Bank2 sector protection option status byte Setting WRPSG2 bits to 0 write protects the corresponding sectors in bank 2 (0: write protected; 1: not write protected)"]
    #[inline(always)]
    pub fn wrpsg2(&self) -> WRPSG2_R {
        WRPSG2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bank2 sector protection option status byte Setting WRPSG2 bits to 0 write protects the corresponding sectors in bank 2 (0: write protected; 1: not write protected)"]
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
#[doc = "FLASH write sector protection for Bank2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrpsgn2r_prg](index.html) module"]
pub struct WRPSGN2R_PRG_SPEC;
impl crate::RegisterSpec for WRPSGN2R_PRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrpsgn2r_prg::R](R) reader structure"]
impl crate::Readable for WRPSGN2R_PRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrpsgn2r_prg::W](W) writer structure"]
impl crate::Writable for WRPSGN2R_PRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRPSGN2R_PRG to value 0"]
impl crate::Resettable for WRPSGN2R_PRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
