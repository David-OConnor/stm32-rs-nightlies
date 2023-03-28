#[doc = "Register `OTFDEC_R1STARTADDR` reader"]
pub struct R(crate::R<OTFDEC_R1STARTADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTFDEC_R1STARTADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTFDEC_R1STARTADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTFDEC_R1STARTADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTFDEC_R1STARTADDR` writer"]
pub struct W(crate::W<OTFDEC_R1STARTADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTFDEC_R1STARTADDR_SPEC>;
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
impl From<crate::W<OTFDEC_R1STARTADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTFDEC_R1STARTADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGx_START_ADDR` reader - Region AHB start address This register must be written before the region corresponding REG_EN bit in the OTFDEC_RxCFGR register is set. Writing to this register is discarded if performed while the region CONFIGLOCK bit in the OTFDEC_RxCFGR register is set. Note: When determining the region the first 12 bits (lsb) and the last 4 bits (msb) are ignored. When this register is accessed in read the 4 msb bits and the 12 lsb bits return zeros ."]
pub type REGX_START_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REGx_START_ADDR` writer - Region AHB start address This register must be written before the region corresponding REG_EN bit in the OTFDEC_RxCFGR register is set. Writing to this register is discarded if performed while the region CONFIGLOCK bit in the OTFDEC_RxCFGR register is set. Note: When determining the region the first 12 bits (lsb) and the last 4 bits (msb) are ignored. When this register is accessed in read the 4 msb bits and the 12 lsb bits return zeros ."]
pub type REGX_START_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTFDEC_R1STARTADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Region AHB start address This register must be written before the region corresponding REG_EN bit in the OTFDEC_RxCFGR register is set. Writing to this register is discarded if performed while the region CONFIGLOCK bit in the OTFDEC_RxCFGR register is set. Note: When determining the region the first 12 bits (lsb) and the last 4 bits (msb) are ignored. When this register is accessed in read the 4 msb bits and the 12 lsb bits return zeros ."]
    #[inline(always)]
    pub fn regx_start_addr(&self) -> REGX_START_ADDR_R {
        REGX_START_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region AHB start address This register must be written before the region corresponding REG_EN bit in the OTFDEC_RxCFGR register is set. Writing to this register is discarded if performed while the region CONFIGLOCK bit in the OTFDEC_RxCFGR register is set. Note: When determining the region the first 12 bits (lsb) and the last 4 bits (msb) are ignored. When this register is accessed in read the 4 msb bits and the 12 lsb bits return zeros ."]
    #[inline(always)]
    #[must_use]
    pub fn regx_start_addr(&mut self) -> REGX_START_ADDR_W<0> {
        REGX_START_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTFDEC region 1 start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otfdec_r1startaddr](index.html) module"]
pub struct OTFDEC_R1STARTADDR_SPEC;
impl crate::RegisterSpec for OTFDEC_R1STARTADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otfdec_r1startaddr::R](R) reader structure"]
impl crate::Readable for OTFDEC_R1STARTADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otfdec_r1startaddr::W](W) writer structure"]
impl crate::Writable for OTFDEC_R1STARTADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTFDEC_R1STARTADDR to value 0"]
impl crate::Resettable for OTFDEC_R1STARTADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
