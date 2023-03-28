#[doc = "Register `DCR4` reader"]
pub struct R(crate::R<DCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCR4` writer"]
pub struct W(crate::W<DCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCR4_SPEC>;
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
impl From<crate::W<DCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEF` reader - Transfer error flag"]
pub type TEF_R = crate::BitReader<bool>;
#[doc = "Field `TEF` writer - Transfer error flag"]
pub type TEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCR4_SPEC, bool, O>;
#[doc = "Field `TCF` reader - Transfer complete flag"]
pub type TCF_R = crate::BitReader<bool>;
#[doc = "Field `TCF` writer - Transfer complete flag"]
pub type TCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCR4_SPEC, bool, O>;
#[doc = "Field `FTF` reader - FIFO threshold flag"]
pub type FTF_R = crate::BitReader<bool>;
#[doc = "Field `FTF` writer - FIFO threshold flag"]
pub type FTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCR4_SPEC, bool, O>;
#[doc = "Field `SMF` reader - Status match flag"]
pub type SMF_R = crate::BitReader<bool>;
#[doc = "Field `SMF` writer - Status match flag"]
pub type SMF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCR4_SPEC, bool, O>;
#[doc = "Field `TOF` reader - Timeout flag"]
pub type TOF_R = crate::BitReader<bool>;
#[doc = "Field `TOF` writer - Timeout flag"]
pub type TOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCR4_SPEC, bool, O>;
#[doc = "Field `BUSY` reader - BUSY"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - BUSY"]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCR4_SPEC, bool, O>;
#[doc = "Field `FLEVEL` reader - FIFO level"]
pub type FLEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLEVEL` writer - FIFO level"]
pub type FLEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR4_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0 - Transfer error flag"]
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer complete flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO threshold flag"]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status match flag"]
    #[inline(always)]
    pub fn smf(&self) -> SMF_R {
        SMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timeout flag"]
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:13 - FIFO level"]
    #[inline(always)]
    pub fn flevel(&self) -> FLEVEL_R {
        FLEVEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer error flag"]
    #[inline(always)]
    #[must_use]
    pub fn tef(&mut self) -> TEF_W<0> {
        TEF_W::new(self)
    }
    #[doc = "Bit 1 - Transfer complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<1> {
        TCF_W::new(self)
    }
    #[doc = "Bit 2 - FIFO threshold flag"]
    #[inline(always)]
    #[must_use]
    pub fn ftf(&mut self) -> FTF_W<2> {
        FTF_W::new(self)
    }
    #[doc = "Bit 3 - Status match flag"]
    #[inline(always)]
    #[must_use]
    pub fn smf(&mut self) -> SMF_W<3> {
        SMF_W::new(self)
    }
    #[doc = "Bit 4 - Timeout flag"]
    #[inline(always)]
    #[must_use]
    pub fn tof(&mut self) -> TOF_W<4> {
        TOF_W::new(self)
    }
    #[doc = "Bit 5 - BUSY"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<5> {
        BUSY_W::new(self)
    }
    #[doc = "Bits 8:13 - FIFO level"]
    #[inline(always)]
    #[must_use]
    pub fn flevel(&mut self) -> FLEVEL_W<8> {
        FLEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCR4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr4](index.html) module"]
pub struct DCR4_SPEC;
impl crate::RegisterSpec for DCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcr4::R](R) reader structure"]
impl crate::Readable for DCR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcr4::W](W) writer structure"]
impl crate::Writable for DCR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCR4 to value 0"]
impl crate::Resettable for DCR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
