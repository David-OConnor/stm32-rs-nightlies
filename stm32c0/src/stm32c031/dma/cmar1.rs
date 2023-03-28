#[doc = "Register `CMAR1` reader"]
pub struct R(crate::R<CMAR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMAR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMAR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMAR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMAR1` writer"]
pub struct W(crate::W<CMAR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMAR1_SPEC>;
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
impl From<crate::W<CMAR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMAR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MA` reader - peripheral address It contains the base address of the memory from/to which the data will be read/written. When MSIZE\\[1:0\\]
= 01 (16 bits), bit 0 of MA\\[31:0\\]
is ignored. Access is automatically aligned to a half-word address. When MSIZE = 10 (32 bits), bits 1 and 0 of MA\\[31:0\\]
are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral source address DIR = 1 and the peripheral destination address if DIR = 0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
pub type MA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MA` writer - peripheral address It contains the base address of the memory from/to which the data will be read/written. When MSIZE\\[1:0\\]
= 01 (16 bits), bit 0 of MA\\[31:0\\]
is ignored. Access is automatically aligned to a half-word address. When MSIZE = 10 (32 bits), bits 1 and 0 of MA\\[31:0\\]
are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral source address DIR = 1 and the peripheral destination address if DIR = 0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
pub type MA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMAR1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - peripheral address It contains the base address of the memory from/to which the data will be read/written. When MSIZE\\[1:0\\]
= 01 (16 bits), bit 0 of MA\\[31:0\\]
is ignored. Access is automatically aligned to a half-word address. When MSIZE = 10 (32 bits), bits 1 and 0 of MA\\[31:0\\]
are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral source address DIR = 1 and the peripheral destination address if DIR = 0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - peripheral address It contains the base address of the memory from/to which the data will be read/written. When MSIZE\\[1:0\\]
= 01 (16 bits), bit 0 of MA\\[31:0\\]
is ignored. Access is automatically aligned to a half-word address. When MSIZE = 10 (32 bits), bits 1 and 0 of MA\\[31:0\\]
are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral source address DIR = 1 and the peripheral destination address if DIR = 0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MA_W<0> {
        MA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel 1 memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmar1](index.html) module"]
pub struct CMAR1_SPEC;
impl crate::RegisterSpec for CMAR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmar1::R](R) reader structure"]
impl crate::Readable for CMAR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmar1::W](W) writer structure"]
impl crate::Writable for CMAR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMAR1 to value 0"]
impl crate::Resettable for CMAR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}