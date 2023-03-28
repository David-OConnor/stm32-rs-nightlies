#[doc = "Register `FIFOR` reader"]
pub struct R(crate::R<FIFOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOR` writer"]
pub struct W(crate::W<FIFOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOR_SPEC>;
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
impl From<crate::W<FIFOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFODATA` reader - Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT=1). The FIFO data occupies 16 entries of 32-bit words."]
pub type FIFODATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FIFODATA` writer - Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT=1). The FIFO data occupies 16 entries of 32-bit words."]
pub type FIFODATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFOR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT=1). The FIFO data occupies 16 entries of 32-bit words."]
    #[inline(always)]
    pub fn fifodata(&self) -> FIFODATA_R {
        FIFODATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT=1). The FIFO data occupies 16 entries of 32-bit words."]
    #[inline(always)]
    #[must_use]
    pub fn fifodata(&mut self) -> FIFODATA_W<0> {
        FIFODATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifor](index.html) module"]
pub struct FIFOR_SPEC;
impl crate::RegisterSpec for FIFOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifor::R](R) reader structure"]
impl crate::Readable for FIFOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifor::W](W) writer structure"]
impl crate::Writable for FIFOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFOR to value 0"]
impl crate::Resettable for FIFOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
