#[doc = "Register `TAMP_ATCR1` reader"]
pub struct R(crate::R<TAMP_ATCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_ATCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_ATCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_ATCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMP_ATCR1` writer"]
pub struct W(crate::W<TAMP_ATCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMP_ATCR1_SPEC>;
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
impl From<crate::W<TAMP_ATCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMP_ATCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMP1AM` reader - Tamper 1 active mode"]
pub type TAMP1AM_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1AM` writer - Tamper 1 active mode"]
pub type TAMP1AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_ATCR1_SPEC, bool, O>;
#[doc = "Field `TAMP2AM` reader - Tamper 2 active mode"]
pub type TAMP2AM_R = crate::BitReader<bool>;
#[doc = "Field `TAMP2AM` writer - Tamper 2 active mode"]
pub type TAMP2AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_ATCR1_SPEC, bool, O>;
#[doc = "Field `TAMP3AM` reader - Tamper 3 active mode"]
pub type TAMP3AM_R = crate::BitReader<bool>;
#[doc = "Field `TAMP3AM` writer - Tamper 3 active mode"]
pub type TAMP3AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_ATCR1_SPEC, bool, O>;
#[doc = "Field `TAMP4AM` reader - Tamper 4 active mode"]
pub type TAMP4AM_R = crate::BitReader<bool>;
#[doc = "Field `TAMP4AM` writer - Tamper 4 active mode"]
pub type TAMP4AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_ATCR1_SPEC, bool, O>;
#[doc = "Field `TAMP5AM` reader - Tamper 5 active mode"]
pub type TAMP5AM_R = crate::BitReader<bool>;
#[doc = "Field `TAMP5AM` writer - Tamper 5 active mode"]
pub type TAMP5AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_ATCR1_SPEC, bool, O>;
#[doc = "Field `TAMP6AM` reader - Tamper 6 active mode"]
pub type TAMP6AM_R = crate::BitReader<bool>;
#[doc = "Field `TAMP6AM` writer - Tamper 6 active mode"]
pub type TAMP6AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_ATCR1_SPEC, bool, O>;
#[doc = "Field `TAMP7AM` reader - Tamper 7 active mode"]
pub type TAMP7AM_R = crate::BitReader<bool>;
#[doc = "Field `TAMP7AM` writer - Tamper 7 active mode"]
pub type TAMP7AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_ATCR1_SPEC, bool, O>;
#[doc = "Field `TAMP8AM` reader - Tamper 8 active mode"]
pub type TAMP8AM_R = crate::BitReader<bool>;
#[doc = "Field `TAMP8AM` writer - Tamper 8 active mode"]
pub type TAMP8AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_ATCR1_SPEC, bool, O>;
#[doc = "Field `ATOSEL1` reader - Active tamper shared output 1 selection The selected output must be available in the package pinout"]
pub type ATOSEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATOSEL1` writer - Active tamper shared output 1 selection The selected output must be available in the package pinout"]
pub type ATOSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_ATCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ATOSEL2` reader - Active tamper shared output 2 selection The selected output must be available in the package pinout"]
pub type ATOSEL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATOSEL2` writer - Active tamper shared output 2 selection The selected output must be available in the package pinout"]
pub type ATOSEL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_ATCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ATOSEL3` reader - Active tamper shared output 3 selection The selected output must be available in the package pinout"]
pub type ATOSEL3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATOSEL3` writer - Active tamper shared output 3 selection The selected output must be available in the package pinout"]
pub type ATOSEL3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_ATCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ATOSEL4` reader - Active tamper shared output 4 selection The selected output must be available in the package pinout."]
pub type ATOSEL4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATOSEL4` writer - Active tamper shared output 4 selection The selected output must be available in the package pinout."]
pub type ATOSEL4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_ATCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ATCKSEL` reader - Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output.The selected clock is CK_ATPRE. fCK_ATPRE = fRTCCLK / 2ATCKSEL when (PREDIV_A+1) = 128. ... These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 ck_atpre cycles after all the active tampers are disable."]
pub type ATCKSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATCKSEL` writer - Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output.The selected clock is CK_ATPRE. fCK_ATPRE = fRTCCLK / 2ATCKSEL when (PREDIV_A+1) = 128. ... These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 ck_atpre cycles after all the active tampers are disable."]
pub type ATCKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_ATCR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `ATPER` reader - Active tamper output change period The tamper output is changed every CK_ATPER = (2ATPER x CK_ATPRE) cycles. Refer to ."]
pub type ATPER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATPER` writer - Active tamper output change period The tamper output is changed every CK_ATPER = (2ATPER x CK_ATPRE) cycles. Refer to ."]
pub type ATPER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_ATCR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `ATOSHARE` reader - Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6 TAMP_IN7 is compared with TAMPOUTSEL7 TAMP_IN8 is compared with TAMPOUTSEL8"]
pub type ATOSHARE_R = crate::BitReader<bool>;
#[doc = "Field `ATOSHARE` writer - Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6 TAMP_IN7 is compared with TAMPOUTSEL7 TAMP_IN8 is compared with TAMPOUTSEL8"]
pub type ATOSHARE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_ATCR1_SPEC, bool, O>;
#[doc = "Field `FLTEN` reader - Active tamper filter enable"]
pub type FLTEN_R = crate::BitReader<bool>;
#[doc = "Field `FLTEN` writer - Active tamper filter enable"]
pub type FLTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_ATCR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Tamper 1 active mode"]
    #[inline(always)]
    pub fn tamp1am(&self) -> TAMP1AM_R {
        TAMP1AM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper 2 active mode"]
    #[inline(always)]
    pub fn tamp2am(&self) -> TAMP2AM_R {
        TAMP2AM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper 3 active mode"]
    #[inline(always)]
    pub fn tamp3am(&self) -> TAMP3AM_R {
        TAMP3AM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tamper 4 active mode"]
    #[inline(always)]
    pub fn tamp4am(&self) -> TAMP4AM_R {
        TAMP4AM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tamper 5 active mode"]
    #[inline(always)]
    pub fn tamp5am(&self) -> TAMP5AM_R {
        TAMP5AM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tamper 6 active mode"]
    #[inline(always)]
    pub fn tamp6am(&self) -> TAMP6AM_R {
        TAMP6AM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Tamper 7 active mode"]
    #[inline(always)]
    pub fn tamp7am(&self) -> TAMP7AM_R {
        TAMP7AM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tamper 8 active mode"]
    #[inline(always)]
    pub fn tamp8am(&self) -> TAMP8AM_R {
        TAMP8AM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Active tamper shared output 1 selection The selected output must be available in the package pinout"]
    #[inline(always)]
    pub fn atosel1(&self) -> ATOSEL1_R {
        ATOSEL1_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Active tamper shared output 2 selection The selected output must be available in the package pinout"]
    #[inline(always)]
    pub fn atosel2(&self) -> ATOSEL2_R {
        ATOSEL2_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Active tamper shared output 3 selection The selected output must be available in the package pinout"]
    #[inline(always)]
    pub fn atosel3(&self) -> ATOSEL3_R {
        ATOSEL3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Active tamper shared output 4 selection The selected output must be available in the package pinout."]
    #[inline(always)]
    pub fn atosel4(&self) -> ATOSEL4_R {
        ATOSEL4_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output.The selected clock is CK_ATPRE. fCK_ATPRE = fRTCCLK / 2ATCKSEL when (PREDIV_A+1) = 128. ... These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 ck_atpre cycles after all the active tampers are disable."]
    #[inline(always)]
    pub fn atcksel(&self) -> ATCKSEL_R {
        ATCKSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Active tamper output change period The tamper output is changed every CK_ATPER = (2ATPER x CK_ATPRE) cycles. Refer to ."]
    #[inline(always)]
    pub fn atper(&self) -> ATPER_R {
        ATPER_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 30 - Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6 TAMP_IN7 is compared with TAMPOUTSEL7 TAMP_IN8 is compared with TAMPOUTSEL8"]
    #[inline(always)]
    pub fn atoshare(&self) -> ATOSHARE_R {
        ATOSHARE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Active tamper filter enable"]
    #[inline(always)]
    pub fn flten(&self) -> FLTEN_R {
        FLTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper 1 active mode"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1am(&mut self) -> TAMP1AM_W<0> {
        TAMP1AM_W::new(self)
    }
    #[doc = "Bit 1 - Tamper 2 active mode"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2am(&mut self) -> TAMP2AM_W<1> {
        TAMP2AM_W::new(self)
    }
    #[doc = "Bit 2 - Tamper 3 active mode"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3am(&mut self) -> TAMP3AM_W<2> {
        TAMP3AM_W::new(self)
    }
    #[doc = "Bit 3 - Tamper 4 active mode"]
    #[inline(always)]
    #[must_use]
    pub fn tamp4am(&mut self) -> TAMP4AM_W<3> {
        TAMP4AM_W::new(self)
    }
    #[doc = "Bit 4 - Tamper 5 active mode"]
    #[inline(always)]
    #[must_use]
    pub fn tamp5am(&mut self) -> TAMP5AM_W<4> {
        TAMP5AM_W::new(self)
    }
    #[doc = "Bit 5 - Tamper 6 active mode"]
    #[inline(always)]
    #[must_use]
    pub fn tamp6am(&mut self) -> TAMP6AM_W<5> {
        TAMP6AM_W::new(self)
    }
    #[doc = "Bit 6 - Tamper 7 active mode"]
    #[inline(always)]
    #[must_use]
    pub fn tamp7am(&mut self) -> TAMP7AM_W<6> {
        TAMP7AM_W::new(self)
    }
    #[doc = "Bit 7 - Tamper 8 active mode"]
    #[inline(always)]
    #[must_use]
    pub fn tamp8am(&mut self) -> TAMP8AM_W<7> {
        TAMP8AM_W::new(self)
    }
    #[doc = "Bits 8:9 - Active tamper shared output 1 selection The selected output must be available in the package pinout"]
    #[inline(always)]
    #[must_use]
    pub fn atosel1(&mut self) -> ATOSEL1_W<8> {
        ATOSEL1_W::new(self)
    }
    #[doc = "Bits 10:11 - Active tamper shared output 2 selection The selected output must be available in the package pinout"]
    #[inline(always)]
    #[must_use]
    pub fn atosel2(&mut self) -> ATOSEL2_W<10> {
        ATOSEL2_W::new(self)
    }
    #[doc = "Bits 12:13 - Active tamper shared output 3 selection The selected output must be available in the package pinout"]
    #[inline(always)]
    #[must_use]
    pub fn atosel3(&mut self) -> ATOSEL3_W<12> {
        ATOSEL3_W::new(self)
    }
    #[doc = "Bits 14:15 - Active tamper shared output 4 selection The selected output must be available in the package pinout."]
    #[inline(always)]
    #[must_use]
    pub fn atosel4(&mut self) -> ATOSEL4_W<14> {
        ATOSEL4_W::new(self)
    }
    #[doc = "Bits 16:18 - Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output.The selected clock is CK_ATPRE. fCK_ATPRE = fRTCCLK / 2ATCKSEL when (PREDIV_A+1) = 128. ... These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 ck_atpre cycles after all the active tampers are disable."]
    #[inline(always)]
    #[must_use]
    pub fn atcksel(&mut self) -> ATCKSEL_W<16> {
        ATCKSEL_W::new(self)
    }
    #[doc = "Bits 24:26 - Active tamper output change period The tamper output is changed every CK_ATPER = (2ATPER x CK_ATPRE) cycles. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn atper(&mut self) -> ATPER_W<24> {
        ATPER_W::new(self)
    }
    #[doc = "Bit 30 - Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6 TAMP_IN7 is compared with TAMPOUTSEL7 TAMP_IN8 is compared with TAMPOUTSEL8"]
    #[inline(always)]
    #[must_use]
    pub fn atoshare(&mut self) -> ATOSHARE_W<30> {
        ATOSHARE_W::new(self)
    }
    #[doc = "Bit 31 - Active tamper filter enable"]
    #[inline(always)]
    #[must_use]
    pub fn flten(&mut self) -> FLTEN_W<31> {
        FLTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP active tamper control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_atcr1](index.html) module"]
pub struct TAMP_ATCR1_SPEC;
impl crate::RegisterSpec for TAMP_ATCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamp_atcr1::R](R) reader structure"]
impl crate::Readable for TAMP_ATCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tamp_atcr1::W](W) writer structure"]
impl crate::Writable for TAMP_ATCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAMP_ATCR1 to value 0x0007_0000"]
impl crate::Resettable for TAMP_ATCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0000;
}
