#[doc = "Register `OCTOSPI_WPCCR` reader"]
pub struct R(crate::R<OCTOSPI_WPCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCTOSPI_WPCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCTOSPI_WPCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCTOSPI_WPCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCTOSPI_WPCCR` writer"]
pub struct W(crate::W<OCTOSPI_WPCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCTOSPI_WPCCR_SPEC>;
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
impl From<crate::W<OCTOSPI_WPCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCTOSPI_WPCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMODE` reader - Instruction mode This field defines the instruction phase mode of operation. 101-111: Reserved"]
pub type IMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IMODE` writer - Instruction mode This field defines the instruction phase mode of operation. 101-111: Reserved"]
pub type IMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OCTOSPI_WPCCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `IDTR` reader - Instruction double transfer rate This bit sets the DTR mode for the instruction phase."]
pub type IDTR_R = crate::BitReader<bool>;
#[doc = "Field `IDTR` writer - Instruction double transfer rate This bit sets the DTR mode for the instruction phase."]
pub type IDTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_WPCCR_SPEC, bool, O>;
#[doc = "Field `ISIZE` reader - Instruction size This field defines instruction size."]
pub type ISIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISIZE` writer - Instruction size This field defines instruction size."]
pub type ISIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OCTOSPI_WPCCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADMODE` reader - Address mode This field defines the address phase mode of operation. 101-111: Reserved"]
pub type ADMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADMODE` writer - Address mode This field defines the address phase mode of operation. 101-111: Reserved"]
pub type ADMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OCTOSPI_WPCCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `ADDTR` reader - Address double transfer rate This bit sets the DTR mode for the address phase."]
pub type ADDTR_R = crate::BitReader<bool>;
#[doc = "Field `ADDTR` writer - Address double transfer rate This bit sets the DTR mode for the address phase."]
pub type ADDTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_WPCCR_SPEC, bool, O>;
#[doc = "Field `ADSIZE` reader - Address size This field defines address size."]
pub type ADSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADSIZE` writer - Address size This field defines address size."]
pub type ADSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OCTOSPI_WPCCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ABMODE` reader - Alternate-byte mode This field defines the alternate byte phase mode of operation. 101-111: Reserved"]
pub type ABMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ABMODE` writer - Alternate-byte mode This field defines the alternate byte phase mode of operation. 101-111: Reserved"]
pub type ABMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OCTOSPI_WPCCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `ABDTR` reader - Alternate bytes double transfer rate This bit sets the DTR mode for the alternate bytes phase."]
pub type ABDTR_R = crate::BitReader<bool>;
#[doc = "Field `ABDTR` writer - Alternate bytes double transfer rate This bit sets the DTR mode for the alternate bytes phase."]
pub type ABDTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_WPCCR_SPEC, bool, O>;
#[doc = "Field `ABSIZE` reader - Alternate bytes size This bit defines alternate bytes size."]
pub type ABSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ABSIZE` writer - Alternate bytes size This bit defines alternate bytes size."]
pub type ABSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OCTOSPI_WPCCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DMODE` reader - Data mode This field defines the data phase mode of operation. 101-111: Reserved"]
pub type DMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMODE` writer - Data mode This field defines the data phase mode of operation. 101-111: Reserved"]
pub type DMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OCTOSPI_WPCCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DDTR` reader - Data double transfer rate This bit sets the DTR mode for the data phase."]
pub type DDTR_R = crate::BitReader<bool>;
#[doc = "Field `DDTR` writer - Data double transfer rate This bit sets the DTR mode for the data phase."]
pub type DDTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_WPCCR_SPEC, bool, O>;
#[doc = "Field `DQSE` reader - DQS enable This bit enables the data strobe management."]
pub type DQSE_R = crate::BitReader<bool>;
#[doc = "Field `DQSE` writer - DQS enable This bit enables the data strobe management."]
pub type DQSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_WPCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Instruction mode This field defines the instruction phase mode of operation. 101-111: Reserved"]
    #[inline(always)]
    pub fn imode(&self) -> IMODE_R {
        IMODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Instruction double transfer rate This bit sets the DTR mode for the instruction phase."]
    #[inline(always)]
    pub fn idtr(&self) -> IDTR_R {
        IDTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Instruction size This field defines instruction size."]
    #[inline(always)]
    pub fn isize(&self) -> ISIZE_R {
        ISIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Address mode This field defines the address phase mode of operation. 101-111: Reserved"]
    #[inline(always)]
    pub fn admode(&self) -> ADMODE_R {
        ADMODE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Address double transfer rate This bit sets the DTR mode for the address phase."]
    #[inline(always)]
    pub fn addtr(&self) -> ADDTR_R {
        ADDTR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Address size This field defines address size."]
    #[inline(always)]
    pub fn adsize(&self) -> ADSIZE_R {
        ADSIZE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Alternate-byte mode This field defines the alternate byte phase mode of operation. 101-111: Reserved"]
    #[inline(always)]
    pub fn abmode(&self) -> ABMODE_R {
        ABMODE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Alternate bytes double transfer rate This bit sets the DTR mode for the alternate bytes phase."]
    #[inline(always)]
    pub fn abdtr(&self) -> ABDTR_R {
        ABDTR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Alternate bytes size This bit defines alternate bytes size."]
    #[inline(always)]
    pub fn absize(&self) -> ABSIZE_R {
        ABSIZE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Data mode This field defines the data phase mode of operation. 101-111: Reserved"]
    #[inline(always)]
    pub fn dmode(&self) -> DMODE_R {
        DMODE_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Data double transfer rate This bit sets the DTR mode for the data phase."]
    #[inline(always)]
    pub fn ddtr(&self) -> DDTR_R {
        DDTR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - DQS enable This bit enables the data strobe management."]
    #[inline(always)]
    pub fn dqse(&self) -> DQSE_R {
        DQSE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Instruction mode This field defines the instruction phase mode of operation. 101-111: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn imode(&mut self) -> IMODE_W<0> {
        IMODE_W::new(self)
    }
    #[doc = "Bit 3 - Instruction double transfer rate This bit sets the DTR mode for the instruction phase."]
    #[inline(always)]
    #[must_use]
    pub fn idtr(&mut self) -> IDTR_W<3> {
        IDTR_W::new(self)
    }
    #[doc = "Bits 4:5 - Instruction size This field defines instruction size."]
    #[inline(always)]
    #[must_use]
    pub fn isize(&mut self) -> ISIZE_W<4> {
        ISIZE_W::new(self)
    }
    #[doc = "Bits 8:10 - Address mode This field defines the address phase mode of operation. 101-111: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn admode(&mut self) -> ADMODE_W<8> {
        ADMODE_W::new(self)
    }
    #[doc = "Bit 11 - Address double transfer rate This bit sets the DTR mode for the address phase."]
    #[inline(always)]
    #[must_use]
    pub fn addtr(&mut self) -> ADDTR_W<11> {
        ADDTR_W::new(self)
    }
    #[doc = "Bits 12:13 - Address size This field defines address size."]
    #[inline(always)]
    #[must_use]
    pub fn adsize(&mut self) -> ADSIZE_W<12> {
        ADSIZE_W::new(self)
    }
    #[doc = "Bits 16:18 - Alternate-byte mode This field defines the alternate byte phase mode of operation. 101-111: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn abmode(&mut self) -> ABMODE_W<16> {
        ABMODE_W::new(self)
    }
    #[doc = "Bit 19 - Alternate bytes double transfer rate This bit sets the DTR mode for the alternate bytes phase."]
    #[inline(always)]
    #[must_use]
    pub fn abdtr(&mut self) -> ABDTR_W<19> {
        ABDTR_W::new(self)
    }
    #[doc = "Bits 20:21 - Alternate bytes size This bit defines alternate bytes size."]
    #[inline(always)]
    #[must_use]
    pub fn absize(&mut self) -> ABSIZE_W<20> {
        ABSIZE_W::new(self)
    }
    #[doc = "Bits 24:26 - Data mode This field defines the data phase mode of operation. 101-111: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn dmode(&mut self) -> DMODE_W<24> {
        DMODE_W::new(self)
    }
    #[doc = "Bit 27 - Data double transfer rate This bit sets the DTR mode for the data phase."]
    #[inline(always)]
    #[must_use]
    pub fn ddtr(&mut self) -> DDTR_W<27> {
        DDTR_W::new(self)
    }
    #[doc = "Bit 29 - DQS enable This bit enables the data strobe management."]
    #[inline(always)]
    #[must_use]
    pub fn dqse(&mut self) -> DQSE_W<29> {
        DQSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OCTOSPI wrap communication configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octospi_wpccr](index.html) module"]
pub struct OCTOSPI_WPCCR_SPEC;
impl crate::RegisterSpec for OCTOSPI_WPCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [octospi_wpccr::R](R) reader structure"]
impl crate::Readable for OCTOSPI_WPCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [octospi_wpccr::W](W) writer structure"]
impl crate::Writable for OCTOSPI_WPCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCTOSPI_WPCCR to value 0"]
impl crate::Resettable for OCTOSPI_WPCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}