#[doc = "Register `PKA_CR` reader"]
pub struct R(crate::R<PKA_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKA_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKA_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKA_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PKA_CR` writer"]
pub struct W(crate::W<PKA_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PKA_CR_SPEC>;
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
impl From<crate::W<PKA_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PKA_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - PKA enable. When an illegal operation is selected while EN=1 OPERRF bit is set in PKA_SR. See PKA_CR.MODE bitfield for details. When EN=0 PKA RAM can still be accessed by the application."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - PKA enable. When an illegal operation is selected while EN=1 OPERRF bit is set in PKA_SR. See PKA_CR.MODE bitfield for details. When EN=0 PKA RAM can still be accessed by the application."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PKA_CR_SPEC, bool, O>;
#[doc = "Field `START` reader - start the operation Writing 1 to this bit starts the operation which is selected by MODE\\[5:0\\], using the operands and data already written to the PKA RAM. This bit is always read as 0. When an illegal operation is selected while START bit is set no operation is started, and OPERRF bit is set in PKA_SR. START is ignored if PKA is busy."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - start the operation Writing 1 to this bit starts the operation which is selected by MODE\\[5:0\\], using the operands and data already written to the PKA RAM. This bit is always read as 0. When an illegal operation is selected while START bit is set no operation is started, and OPERRF bit is set in PKA_SR. START is ignored if PKA is busy."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, PKA_CR_SPEC, bool, O>;
#[doc = "Field `MODE` reader - PKA operation code When an operation not listed here is written by the application with EN bit set, OPERRF bit is set in PKA_SR register, and the write to MODE bitfield is ignored. When PKA is configured in limited mode (LMF = 1 in PKA_SR), writing a MODE different from 0x26 with EN bit to 1 triggers OPERRF bit to be set and write to MODE bit is ignored."]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - PKA operation code When an operation not listed here is written by the application with EN bit set, OPERRF bit is set in PKA_SR register, and the write to MODE bitfield is ignored. When PKA is configured in limited mode (LMF = 1 in PKA_SR), writing a MODE different from 0x26 with EN bit to 1 triggers OPERRF bit to be set and write to MODE bit is ignored."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PKA_CR_SPEC, u8, u8, 6, O>;
#[doc = "Field `PROCENDIE` reader - End of operation interrupt enable"]
pub type PROCENDIE_R = crate::BitReader<bool>;
#[doc = "Field `PROCENDIE` writer - End of operation interrupt enable"]
pub type PROCENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PKA_CR_SPEC, bool, O>;
#[doc = "Field `RAMERRIE` reader - RAM error interrupt enable"]
pub type RAMERRIE_R = crate::BitReader<bool>;
#[doc = "Field `RAMERRIE` writer - RAM error interrupt enable"]
pub type RAMERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PKA_CR_SPEC, bool, O>;
#[doc = "Field `ADDRERRIE` reader - Address error interrupt enable"]
pub type ADDRERRIE_R = crate::BitReader<bool>;
#[doc = "Field `ADDRERRIE` writer - Address error interrupt enable"]
pub type ADDRERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PKA_CR_SPEC, bool, O>;
#[doc = "Field `OPERRIE` reader - Operation error interrupt enable"]
pub type OPERRIE_R = crate::BitReader<bool>;
#[doc = "Field `OPERRIE` writer - Operation error interrupt enable"]
pub type OPERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PKA_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PKA enable. When an illegal operation is selected while EN=1 OPERRF bit is set in PKA_SR. See PKA_CR.MODE bitfield for details. When EN=0 PKA RAM can still be accessed by the application."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - start the operation Writing 1 to this bit starts the operation which is selected by MODE\\[5:0\\], using the operands and data already written to the PKA RAM. This bit is always read as 0. When an illegal operation is selected while START bit is set no operation is started, and OPERRF bit is set in PKA_SR. START is ignored if PKA is busy."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:13 - PKA operation code When an operation not listed here is written by the application with EN bit set, OPERRF bit is set in PKA_SR register, and the write to MODE bitfield is ignored. When PKA is configured in limited mode (LMF = 1 in PKA_SR), writing a MODE different from 0x26 with EN bit to 1 triggers OPERRF bit to be set and write to MODE bit is ignored."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 17 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn procendie(&self) -> PROCENDIE_R {
        PROCENDIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - RAM error interrupt enable"]
    #[inline(always)]
    pub fn ramerrie(&self) -> RAMERRIE_R {
        RAMERRIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Address error interrupt enable"]
    #[inline(always)]
    pub fn addrerrie(&self) -> ADDRERRIE_R {
        ADDRERRIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Operation error interrupt enable"]
    #[inline(always)]
    pub fn operrie(&self) -> OPERRIE_R {
        OPERRIE_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PKA enable. When an illegal operation is selected while EN=1 OPERRF bit is set in PKA_SR. See PKA_CR.MODE bitfield for details. When EN=0 PKA RAM can still be accessed by the application."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - start the operation Writing 1 to this bit starts the operation which is selected by MODE\\[5:0\\], using the operands and data already written to the PKA RAM. This bit is always read as 0. When an illegal operation is selected while START bit is set no operation is started, and OPERRF bit is set in PKA_SR. START is ignored if PKA is busy."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<1> {
        START_W::new(self)
    }
    #[doc = "Bits 8:13 - PKA operation code When an operation not listed here is written by the application with EN bit set, OPERRF bit is set in PKA_SR register, and the write to MODE bitfield is ignored. When PKA is configured in limited mode (LMF = 1 in PKA_SR), writing a MODE different from 0x26 with EN bit to 1 triggers OPERRF bit to be set and write to MODE bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<8> {
        MODE_W::new(self)
    }
    #[doc = "Bit 17 - End of operation interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn procendie(&mut self) -> PROCENDIE_W<17> {
        PROCENDIE_W::new(self)
    }
    #[doc = "Bit 19 - RAM error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ramerrie(&mut self) -> RAMERRIE_W<19> {
        RAMERRIE_W::new(self)
    }
    #[doc = "Bit 20 - Address error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn addrerrie(&mut self) -> ADDRERRIE_W<20> {
        ADDRERRIE_W::new(self)
    }
    #[doc = "Bit 21 - Operation error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn operrie(&mut self) -> OPERRIE_W<21> {
        OPERRIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PKA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pka_cr](index.html) module"]
pub struct PKA_CR_SPEC;
impl crate::RegisterSpec for PKA_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pka_cr::R](R) reader structure"]
impl crate::Readable for PKA_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pka_cr::W](W) writer structure"]
impl crate::Writable for PKA_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PKA_CR to value 0"]
impl crate::Resettable for PKA_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
